// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use crate::{RenameTable, StaticSingleAssigner};

use leo_ast::{
    AssignStatement, BinaryExpression, BinaryOperation, Block, ConditionalStatement, DefinitionStatement, Expression,
    ExpressionReconstructor, Identifier, Node, ReturnStatement, Statement, StatementReconstructor, TernaryExpression,
    UnaryExpression, UnaryOperation, ExpressionKind,
};
use leo_span::Symbol;

use indexmap::IndexSet;

impl StatementReconstructor for StaticSingleAssigner<'_> {
    /// Transforms a `ReturnStatement` into an empty `BlockStatement`,
    /// storing the expression and the associated guard in `self.early_returns`.
    ///
    /// Note that type checking guarantees that there is at most one `ReturnStatement` in a block.
    fn reconstruct_return(&mut self, input: ReturnStatement) -> Statement {
        // Construct the associated guard.
        let guard = match self.condition_stack.is_empty() {
            true => None,
            false => {
                let (first, rest) = self.condition_stack.split_first().unwrap();
                Some(rest.iter().cloned().fold(first.clone(), |acc, condition| {
                    let span = acc.span + condition.span;
                    let kind = ExpressionKind::Binary(BinaryExpression {
                        op: BinaryOperation::And,
                        left: Box::new(acc),
                        right: Box::new(condition),
                    });
                    Expression { kind, span }
                }))
            }
        };

        // Reconstruct the expression and add it to the early returns.
        let expression = self.reconstruct_expression(input.expression).0;
        self.early_returns.push((guard, expression));

        // Return an empty block.
        Statement::dummy(input.span)
    }

    /// Reconstructs the `DefinitionStatement` into an `AssignStatement`, renaming the left-hand-side as appropriate.
    fn reconstruct_definition(&mut self, definition: DefinitionStatement) -> Statement {
        self.is_lhs = true;
        let identifier = match self.reconstruct_identifier(definition.variable_name).0.kind {
            ExpressionKind::Identifier(identifier) => identifier,
            _ => unreachable!("`self.reconstruct_identifier` will always return an `Identifier`."),
        };
        self.is_lhs = false;

        let kind = ExpressionKind::Identifier(identifier);
        let span = identifier.span;
        Self::simple_assign_statement(
            Expression { kind, span },
            self.reconstruct_expression(definition.value).0,
        )
    }

    /// Reconstruct all `AssignStatement`s, renaming as necessary.
    fn reconstruct_assign(&mut self, assign: AssignStatement) -> Statement {
        // First reconstruct the right-hand-side of the assignment.
        let value = self.reconstruct_expression(assign.value).0;

        // Then assign a new unique name to the left-hand-side of the assignment.
        // Note that this order is necessary to ensure that the right-hand-side uses the correct name when reconstructing a complex assignment.
        self.is_lhs = true;
        let place = self.reconstruct_expression(assign.place).0;
        self.is_lhs = false;

        Self::simple_assign_statement(place, value)
    }

    /// Reconstructs a `ConditionalStatement`, producing phi functions for variables written in the then-block and otherwise-block.
    fn reconstruct_conditional(&mut self, conditional: ConditionalStatement) -> Statement {
        let condition = self.reconstruct_expression(conditional.condition).0;

        // Instantiate a `RenameTable` for the then-block.
        self.push();

        // Add condition to the condition stack.
        self.condition_stack.push(condition.clone());

        // Reconstruct the then-block.
        let then = self.reconstruct_block(conditional.then);

        // Remove condition from the condition stack.
        self.condition_stack.pop();

        // Remove the `RenameTable` for the then-block.
        let if_table = self.pop();

        // Instantiate a `RenameTable` for the otherwise-block.
        self.push();

        // Reconstruct the otherwise-block.
        let otherwise = conditional.otherwise.map(|statement| {
            // Add the negated condition to the condition stack.
            let span = condition.span();
            let kind = ExpressionKind::Unary(UnaryExpression {
                op: UnaryOperation::Not,
                receiver: Box::new(condition.clone()),
            });
            self.condition_stack.push(Expression { kind, span });

            let reconstructed_block = Box::new(match *statement {
                // The `ConditionalStatement` must be reconstructed as a `Block` statement to ensure that appropriate statements are produced.
                Statement::Conditional(stmt) => self.reconstruct_statement(Statement::Block(Block {
                    statements: vec![Statement::Conditional(stmt)],
                    span: Default::default(),
                })),
                Statement::Block(stmt) => self.reconstruct_statement(Statement::Block(stmt)),
                _ => unreachable!(
                    "`ConditionalStatement`s next statement must be a `ConditionalStatement` or a `Block`."
                ),
            });

            // Remove the negated condition from the condition stack.
            self.condition_stack.pop();

            reconstructed_block
        });

        // Remove the `RenameTable` for the otherwise-block.
        let else_table = self.pop();

        // Compute the write set for the variables written in the then-block or otherwise-block.
        let if_write_set: IndexSet<&Symbol> = IndexSet::from_iter(if_table.local_names());
        let else_write_set: IndexSet<&Symbol> = IndexSet::from_iter(else_table.local_names());
        let write_set = if_write_set.union(&else_write_set);

        // For each variable in the write set, instantiate a phi function.
        for symbol in write_set {
            // Note that phi functions only need to be instantiated if the variable exists before the `ConditionalStatement`.
            if self.rename_table.lookup(**symbol).is_some() {
                let span = <_>::default();
                // Helper to lookup a symbol and create an argument for the phi function.
                let create_phi_argument = |table: &RenameTable, symbol: Symbol| {
                    let name = *table
                        .lookup(symbol)
                        .unwrap_or_else(|| panic!("Symbol {} should exist in the program.", symbol));
                    let kind = ExpressionKind::Identifier(Identifier::new(name));
                    Box::new(Expression { kind, span })
                };

                // Create a new name for the variable written to in the `ConditionalStatement`.
                let new_name = self.unique_symbol(symbol);

                // Create a new `AssignStatement` for the phi function.
                let assignment = Self::simple_assign_statement(
                    Expression {
                        span,
                        kind: ExpressionKind::Identifier(Identifier::new(new_name)),
                    },
                    Expression {
                        span,
                        kind: ExpressionKind::Ternary(TernaryExpression {
                            condition: Box::new(condition.clone()),
                            if_true: create_phi_argument(&if_table, **symbol),
                            if_false: create_phi_argument(&else_table, **symbol),
                        }),
                    },
                );

                // Update the `RenameTable` with the new name of the variable.
                self.rename_table.update(*(*symbol), new_name);

                // Store the generate phi functions.
                self.phi_functions.push(assignment);
            }
        }

        // Note that we only produce
        Statement::Conditional(ConditionalStatement {
            condition,
            then,
            otherwise,
            span: conditional.span,
        })
    }

    /// Reconstructs a `Block`, flattening its constituent `ConditionalStatement`s.
    fn reconstruct_block(&mut self, block: Block) -> Block {
        let mut statements = Vec::with_capacity(block.statements.len());

        // Reconstruct each statement in the block.
        for statement in block.statements.into_iter() {
            match statement {
                Statement::Conditional(conditional_statement) => {
                    statements.extend(self.flatten_conditional_statement(conditional_statement))
                }
                _ => statements.push(self.reconstruct_statement(statement)),
            }
        }

        Block {
            statements,
            span: block.span,
        }
    }
}
