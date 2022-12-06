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

use crate::{TypeChecker, VariableSymbol, VariableType};
use itertools::Itertools;

use leo_ast::*;
use leo_errors::TypeCheckerError;
use leo_span::{Span, Symbol};

impl<'a> StatementVisitor<'a> for TypeChecker<'a> {
    fn visit_statement(&mut self, input: &'a Statement) {
        // No statements can follow a return statement.
        if self.has_return {
            self.emit_err(TypeCheckerError::unreachable_code_after_return(input.span()));
            return;
        }

        match input {
            Statement::Assert(stmt) => self.visit_assert(stmt),
            Statement::Assign(stmt) => self.visit_assign(stmt),
            Statement::Block(stmt) => self.visit_block(stmt),
            Statement::Conditional(stmt) => self.visit_conditional(stmt),
            Statement::Console(stmt) => self.visit_console(stmt),
            Statement::Decrement(stmt) => self.visit_decrement(stmt),
            Statement::Definition(stmt) => self.visit_definition(stmt),
            Statement::Expression(stmt) => self.visit_expression_statement(stmt),
            Statement::Increment(stmt) => self.visit_increment(stmt),
            Statement::Iteration(stmt) => self.visit_iteration(stmt),
            Statement::Return(stmt) => self.visit_return(stmt),
        }
    }

    fn visit_assert(&mut self, input: &'a AssertStatement) {
        match &input.variant{
            AssertVariant::Assert(expr) => {
                let type_ = self.visit_expression(expr, &Some(Type::Boolean));
                self.assert_bool_type(&type_, expr.span());
            }
            AssertVariant::AssertEq(left, right) | AssertVariant::AssertNeq(left, right) => {
                let t1 = self.visit_expression(left, &None);
                let t2 = self.visit_expression(right, &None);

                // Check that the types are equal.
                self.check_eq_types(&t1, &t2, input.span());
            }
        }
    }

    fn visit_assign(&mut self, input: &'a AssignStatement) {
        let var_name = match input.place {
            Expression::Identifier(id) => id,
            _ => {
                self.emit_err(TypeCheckerError::invalid_assignment_target(input.place.span()));
                return;
            }
        };

        let var_type = if let Some(var) = self.symbol_table.borrow_mut().lookup_variable(var_name.name) {
            match &var.declaration {
                VariableType::Const => self.emit_err(TypeCheckerError::cannot_assign_to_const_var(var_name, var.span)),
                VariableType::Input(Mode::Const) => {
                    self.emit_err(TypeCheckerError::cannot_assign_to_const_input(var_name, var.span))
                }
                _ => {}
            }

            Some(var.type_.clone())
        } else {
            self.emit_err(TypeCheckerError::unknown_sym("variable", var_name.name, var_name.span));

            None
        };

        if var_type.is_some() {
            self.visit_expression(&input.value, &var_type);
        }
    }

    fn visit_block(&mut self, input: &'a Block) {
        // Create a new scope for the then-block.
        let scope_index = self.create_child_scope();

        input.statements.iter().for_each(|stmt| self.visit_statement(stmt));

        // Exit the scope for the then-block.
        self.exit_scope(scope_index);
    }

    fn visit_conditional(&mut self, input: &'a ConditionalStatement) {
        self.visit_expression(&input.condition, &Some(Type::Boolean));

        let mut then_block_has_return = false;
        let mut otherwise_block_has_return = false;

        let mut then_block_has_finalize = false;
        let mut otherwise_block_has_finalize = false;

        // Set the `has_return` flag for the then-block.
        let previous_has_return = core::mem::replace(&mut self.has_return, then_block_has_return);
        // Set the `has_finalize` flag for the then-block.
        let previous_has_finalize = core::mem::replace(&mut self.has_finalize, then_block_has_finalize);

        self.visit_block(&input.then);

        // Store the `has_return` flag for the then-block.
        then_block_has_return = self.has_return;
        // Store the `has_finalize` flag for the then-block.
        then_block_has_finalize = self.has_finalize;

        if let Some(otherwise) = &input.otherwise {
            // Set the `has_return` flag for the otherwise-block.
            self.has_return = otherwise_block_has_return;
            // Set the `has_finalize` flag for the otherwise-block.
            self.has_finalize = otherwise_block_has_finalize;

            match &**otherwise {
                Statement::Block(stmt) => {
                    // Visit the otherwise-block.
                    self.visit_block(stmt);
                }
                Statement::Conditional(stmt) => self.visit_conditional(stmt),
                _ => unreachable!("Else-case can only be a block or conditional statement."),
            }

            // Store the `has_return` flag for the otherwise-block.
            otherwise_block_has_return = self.has_return;
            // Store the `has_finalize` flag for the otherwise-block.
            otherwise_block_has_finalize = self.has_finalize;
        }

        // Restore the previous `has_return` flag.
        self.has_return = previous_has_return || (then_block_has_return && otherwise_block_has_return);
        // Restore the previous `has_finalize` flag.
        self.has_finalize = previous_has_finalize || (then_block_has_finalize && otherwise_block_has_finalize);
    }

    fn visit_console(&mut self, _: &'a ConsoleStatement) {
        unreachable!("Parsing guarantees that console statements are not present in the AST.");
    }

    fn visit_decrement(&mut self, input: &'a DecrementStatement) {
        if !self.is_finalize {
            self.emit_err(TypeCheckerError::increment_or_decrement_outside_finalize(input.span()));
        }

        // Assert that the first operand is a mapping.
        let mapping_type = self.visit_identifier(&input.mapping, &None);
        self.assert_mapping_type(&mapping_type, input.span());

        match mapping_type {
            None => self.emit_err(TypeCheckerError::could_not_determine_type(
                input.mapping,
                input.mapping.span,
            )),
            Some(Type::Mapping(mapping_type)) => {
                // Check that the index matches the key type of the mapping.
                let index_type = self.visit_expression(&input.index, &None);
                self.assert_type(&index_type, &mapping_type.key, input.index.span());

                // Check that the amount matches the value type of the mapping.
                let amount_type = self.visit_expression(&input.amount, &None);
                self.assert_type(&amount_type, &mapping_type.value, input.amount.span());

                // Check that the amount type is incrementable.
                self.assert_field_group_scalar_int_type(&amount_type, input.amount.span());
            }
            Some(mapping_type) => self.emit_err(TypeCheckerError::expected_one_type_of(
                "mapping",
                mapping_type,
                input.mapping.span,
            )),
        }
    }

    fn visit_definition(&mut self, input: &'a DefinitionStatement) {
        let declaration = if input.declaration_type == DeclarationType::Const {
            VariableType::Const
        } else {
            VariableType::Mut
        };

        // Check that the type of the definition is defined.
        self.assert_type_is_defined(&input.type_, input.span);

        // Check that the type of the definition is not a unit type, singleton tuple type, or nested tuple type.
        match &input.type_ {
            // If the type is an empty tuple, return an error.
            Type::Unit => self.emit_err(TypeCheckerError::lhs_must_be_identifier_or_tuple(input.span)),
            // If the type is a singleton tuple, return an error.
            Type::Tuple(tuple) => match tuple.len() {
                0 | 1 => unreachable!("Parsing guarantees that tuple types have at least two elements."),
                _ => {
                    if tuple.iter().any(|type_| matches!(type_, Type::Tuple(_))) {
                        self.emit_err(TypeCheckerError::nested_tuple_type(input.span))
                    }
                }
            },
            Type::Mapping(_) | Type::Err => unreachable!(),
            // Otherwise, the type is valid.
            _ => (), // Do nothing
        }

        // Check the expression on the left-hand side.
        self.visit_expression(&input.value, &Some(input.type_.clone()));

        // TODO: Dedup with unrolling pass.
        // Helper to insert the variables into the symbol table.
        let insert_variable = |symbol: Symbol, type_: Type, span: Span, declaration: VariableType| {
            if let Err(err) = self.symbol_table.borrow_mut().insert_variable(
                symbol,
                VariableSymbol {
                    type_,
                    span,
                    declaration,
                },
            ) {
                self.handler.emit_err(err);
            }
        };

        // Insert the variables in the into the symbol table.
        match &input.place {
            Expression::Identifier(identifier) => {
                insert_variable(identifier.name, input.type_.clone(), identifier.span, declaration)
            }
            Expression::Tuple(tuple_expression) => {
                let tuple_type = match &input.type_ {
                    Type::Tuple(tuple_type) => tuple_type,
                    _ => unreachable!(
                        "Type checking guarantees that if the lhs is a tuple, its associated type is also a tuple."
                    ),
                };
                tuple_expression
                    .elements
                    .iter()
                    .zip_eq(tuple_type.0.iter())
                    .for_each(|(expression, type_)| {
                        let identifier = match expression {
                            Expression::Identifier(identifier) => identifier,
                            _ => {
                                return self.emit_err(TypeCheckerError::lhs_tuple_element_must_be_an_identifier(
                                    expression.span(),
                                ))
                            }
                        };
                        insert_variable(identifier.name, type_.clone(), identifier.span, declaration)
                    });
            }
            _ => self.emit_err(TypeCheckerError::lhs_must_be_identifier_or_tuple(input.place.span())),
        }
    }

    fn visit_expression_statement(&mut self, input: &'a ExpressionStatement) {
        // Expression statements can only be function calls.
        if !matches!(input.expression, Expression::Call(_)) {
            self.emit_err(TypeCheckerError::expression_statement_must_be_function_call(
                input.span(),
            ));
        } else {
            // Check the expression.
            // TODO: Should the output type be restricted to unit types?
            self.visit_expression(&input.expression, &None);
        }
    }

    fn visit_increment(&mut self, input: &'a IncrementStatement) {
        if !self.is_finalize {
            self.emit_err(TypeCheckerError::increment_or_decrement_outside_finalize(input.span()));
        }

        // Assert that the first operand is a mapping.
        let mapping_type = self.visit_identifier(&input.mapping, &None);
        self.assert_mapping_type(&mapping_type, input.span());

        match mapping_type {
            None => self.emit_err(TypeCheckerError::could_not_determine_type(
                input.mapping,
                input.mapping.span,
            )),
            Some(Type::Mapping(mapping_type)) => {
                // Check that the index matches the key type of the mapping.
                let index_type = self.visit_expression(&input.index, &None);
                self.assert_type(&index_type, &mapping_type.key, input.index.span());

                // Check that the amount matches the value type of the mapping.
                let amount_type = self.visit_expression(&input.amount, &None);
                self.assert_type(&amount_type, &mapping_type.value, input.amount.span());

                // Check that the amount type is incrementable.
                self.assert_field_group_scalar_int_type(&amount_type, input.amount.span());
            }
            Some(mapping_type) => self.emit_err(TypeCheckerError::expected_one_type_of(
                "mapping",
                mapping_type,
                input.mapping.span,
            )),
        }
    }

    fn visit_iteration(&mut self, input: &'a IterationStatement) {
        let iter_type = &Some(input.type_.clone());
        self.assert_int_type(iter_type, input.variable.span);

        // Create a new scope for the loop body.
        let scope_index = self.create_child_scope();

        // Add the loop variable to the scope of the loop body.
        if let Err(err) = self.symbol_table.borrow_mut().insert_variable(
            input.variable.name,
            VariableSymbol {
                type_: input.type_.clone(),
                span: input.span(),
                declaration: VariableType::Const,
            },
        ) {
            self.handler.emit_err(err);
        }

        let prior_has_return = core::mem::take(&mut self.has_return);
        let prior_has_finalize = core::mem::take(&mut self.has_finalize);

        self.visit_block(&input.block);

        if self.has_return {
            self.emit_err(TypeCheckerError::loop_body_contains_return(input.span()));
        }

        if self.has_finalize {
            self.emit_err(TypeCheckerError::loop_body_contains_finalize(input.span()));
        }

        self.has_return = prior_has_return;
        self.has_finalize = prior_has_finalize;

        // Exit the scope.
        self.exit_scope(scope_index);

        self.visit_expression(&input.start, iter_type);

        // If `input.start` is a literal, instantiate it as a value.
        if let Expression::Literal(literal) = &input.start {
            input.start_value.replace(Some(Value::from(literal)));
        } else {
            self.emit_err(TypeCheckerError::loop_bound_must_be_a_literal(input.start.span()));
        }

        self.visit_expression(&input.stop, iter_type);

        // If `input.stop` is a literal, instantiate it as a value.
        if let Expression::Literal(literal) = &input.stop {
            input.stop_value.replace(Some(Value::from(literal)));
        } else {
            self.emit_err(TypeCheckerError::loop_bound_must_be_a_literal(input.stop.span()));
        }
    }

    fn visit_return(&mut self, input: &'a ReturnStatement) {
        // We can safely unwrap all self.parent instances because
        // statements should always have some parent block
        let parent = self.function.unwrap();
        let return_type = &self
            .symbol_table
            .borrow()
            .lookup_fn_symbol(parent)
            .map(|f| match self.is_finalize {
                // TODO: Check this.
                // Note that this `unwrap()` is safe since we checked that the function has a finalize block.
                true => f.finalize.as_ref().unwrap().output_type.clone(),
                false => f.output_type.clone(),
            });

        // Set the `has_return` flag.
        self.has_return = true;

        // Check that the return expression is not a nested tuple.
        if let Expression::Tuple(TupleExpression { elements, .. }) = &input.expression {
            for element in elements {
                if matches!(element, Expression::Tuple(_)) {
                    self.emit_err(TypeCheckerError::nested_tuple_expression(element.span()));
                }
            }
        }

        // Set the `is_return` flag. This is necessary to allow unit expressions in the return statement.
        self.is_return = true;
        // Type check the associated expression.
        self.visit_expression(&input.expression, return_type);
        // Unset the `is_return` flag.
        self.is_return = false;

        if let Some(arguments) = &input.finalize_arguments {
            if self.is_finalize {
                self.emit_err(TypeCheckerError::finalize_in_finalize(input.span()));
            }

            // Set the `has_finalize` flag.
            self.has_finalize = true;

            // Check that the function has a finalize block.
            // Note that `self.function.unwrap()` is safe since every `self.function` is set for every function.
            // Note that `(self.function.unwrap()).unwrap()` is safe since all functions have been checked to exist.
            let finalize = self
                .symbol_table
                .borrow()
                .lookup_fn_symbol(self.function.unwrap())
                .unwrap()
                .finalize
                .clone();
            match finalize {
                None => self.emit_err(TypeCheckerError::finalize_without_finalize_block(input.span())),
                Some(finalize) => {
                    // Check number of function arguments.
                    if finalize.input.len() != arguments.len() {
                        self.emit_err(TypeCheckerError::incorrect_num_args_to_finalize(
                            finalize.input.len(),
                            arguments.len(),
                            input.span(),
                        ));
                    }

                    // Check function argument types.
                    finalize
                        .input
                        .iter()
                        .zip(arguments.iter())
                        .for_each(|(expected, argument)| {
                            self.visit_expression(argument, &Some(expected.type_()));
                        });
                }
            }
        }
    }
}
