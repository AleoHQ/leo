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

use leo_ast::{
    Block, DeclarationType, DefinitionStatement, Expression, ExpressionKind, IterationStatement, Literal, Statement,
    StatementReconstructor, Type, Value,
};
use std::cell::RefCell;

use leo_errors::emitter::Handler;

use crate::{Clusivity, LoopBound, RangeIterator, SymbolTable};

pub struct Unroller<'a> {
    /// The symbol table for the function being processed.
    pub(crate) symbol_table: RefCell<SymbolTable>,
    /// The index of the current block scope.
    pub(crate) block_index: usize,
    /// An error handler used for any errors found during unrolling.
    pub(crate) handler: &'a Handler,
    /// Are we in the midst of unrolling a loop?
    pub(crate) is_unrolling: bool,
}

impl<'a> Unroller<'a> {
    pub(crate) fn new(symbol_table: SymbolTable, handler: &'a Handler) -> Self {
        Self {
            symbol_table: RefCell::new(symbol_table),
            block_index: 0,
            handler,
            is_unrolling: false,
        }
    }

    /// Returns the index of the current scope.
    /// Note that if we are in the midst of unrolling an IterationStatement, a new scope is created.
    pub(crate) fn current_scope_index(&mut self) -> usize {
        if self.is_unrolling {
            self.symbol_table.borrow_mut().insert_block()
        } else {
            self.block_index
        }
    }

    /// Enters a child block scope.
    pub(crate) fn enter_block_scope(&mut self, index: usize) {
        let previous_symbol_table = std::mem::take(&mut self.symbol_table);
        self.symbol_table
            .swap(previous_symbol_table.borrow().lookup_scope_by_index(index).unwrap());
        self.symbol_table.borrow_mut().parent = Some(Box::new(previous_symbol_table.into_inner()));
    }

    /// Exits the current block scope.
    pub(crate) fn exit_block_scope(&mut self, index: usize) {
        let prev_st = *self.symbol_table.borrow_mut().parent.take().unwrap();
        self.symbol_table.swap(prev_st.lookup_scope_by_index(index).unwrap());
        self.symbol_table = RefCell::new(prev_st);
    }

    /// Unrolls an IterationStatement.
    pub(crate) fn unroll_iteration_statement<I: LoopBound>(
        &mut self,
        input: IterationStatement,
        start: Value,
        stop: Value,
    ) -> Statement {
        // Closure to check that the constant values are valid u128.
        // We already know these are integers since loop unrolling occurs after type checking.
        let cast_to_number = |v: Value| -> Result<I, Statement> {
            match v.try_into() {
                Ok(val_as_u128) => Ok(val_as_u128),
                Err(err) => {
                    self.handler.emit_err(err);
                    Err(Statement::dummy(input.span))
                }
            }
        };

        // Cast `start` to `I`.
        let start = match cast_to_number(start) {
            Ok(v) => v,
            Err(s) => return s,
        };
        // Cast `stop` to `I`.
        let stop = match cast_to_number(stop) {
            Ok(v) => v,
            Err(s) => return s,
        };

        // Get the index of the current scope.
        let scope_index = self.current_scope_index();

        // Enter the scope of the loop body.
        self.enter_block_scope(scope_index);
        self.block_index = 0;

        // Clear the symbol table for the loop body.
        // This is necessary because loop unrolling transforms the program, which requires reconstructing the symbol table.
        self.symbol_table.borrow_mut().variables.clear();
        self.symbol_table.borrow_mut().scopes.clear();
        self.symbol_table.borrow_mut().scope_index = 0;

        // Create a block statement to replace the iteration statement.
        // Creates a new block per iteration inside the outer block statement.
        let iter_blocks = Statement::Block(Block {
            span: input.span,
            statements: match input.inclusive {
                true => {
                    let iter = RangeIterator::new(start, stop, Clusivity::Inclusive);
                    iter.map(|iteration_count| self.unroll_single_iteration(&input, iteration_count))
                        .collect()
                }
                false => {
                    let iter = RangeIterator::new(start, stop, Clusivity::Exclusive);
                    iter.map(|iteration_count| self.unroll_single_iteration(&input, iteration_count))
                        .collect()
                }
            },
        });

        // Exit the scope of the loop body.
        self.exit_block_scope(scope_index);
        self.block_index = scope_index + 1;

        iter_blocks
    }

    /// Unrolls a single iteration of an `IterationStatement`.
    fn unroll_single_iteration<I: LoopBound>(&mut self, input: &IterationStatement, iteration_count: I) -> Statement {
        // Create a scope for a single unrolling of the `IterationStatement`.
        let scope_index = self.symbol_table.borrow_mut().insert_block();
        self.enter_block_scope(scope_index);

        let prior_is_unrolling = self.is_unrolling;
        self.is_unrolling = true;

        // Reconstruct `iteration_count` as a `Literal`.
        let span = input.span;
        let ic_string = iteration_count.to_string();
        let value = match input.type_ {
            Type::I8 => Literal::I8(ic_string, span),
            Type::I16 => Literal::I16(ic_string, span),
            Type::I32 => Literal::I32(ic_string, span),
            Type::I64 => Literal::I64(ic_string, span),
            Type::I128 => Literal::I128(ic_string, span),
            Type::U8 => Literal::U8(ic_string, span),
            Type::U16 => Literal::U16(ic_string, span),
            Type::U32 => Literal::U32(ic_string, span),
            Type::U64 => Literal::U64(ic_string, span),
            Type::U128 => Literal::U128(ic_string, span),
            _ => unreachable!(
                "The iteration variable must be an integer type. This should be enforced by type checking."
            ),
        };

        // The first statement in the block is the assignment of the loop variable to the current iteration count.
        let mut statements = vec![self.reconstruct_definition(DefinitionStatement {
            declaration_type: DeclarationType::Const,
            type_: input.type_.clone(),
            value: Expression {
                span,
                kind: ExpressionKind::Literal(value),
            },
            span: Default::default(),
            variable_name: input.variable,
        })];

        // Reconstruct the statements in the loop body.
        input.block.statements.clone().into_iter().for_each(|s| {
            statements.push(self.reconstruct_statement(s));
        });

        let block = Statement::Block(Block {
            statements,
            span: input.block.span,
        });

        self.is_unrolling = prior_is_unrolling;

        // Exit the scope.
        self.exit_block_scope(scope_index);

        block
    }
}
