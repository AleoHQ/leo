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

use crate::CodeGenerator;

use leo_ast::{
    AssignStatement, Block, ConditionalStatement, ConsoleFunction, ConsoleStatement, DecrementStatement,
    DefinitionStatement, Expression, FinalizeStatement, IncrementStatement, IterationStatement, Mode, Output,
    ReturnStatement, Statement,
};

use itertools::Itertools;
use std::fmt::Write as _;

impl<'a> CodeGenerator<'a> {
    fn visit_statement(&mut self, input: &'a Statement) -> String {
        match input {
            Statement::Assign(stmt) => self.visit_assign(stmt),
            Statement::Block(stmt) => self.visit_block(stmt),
            Statement::Conditional(stmt) => self.visit_conditional(stmt),
            Statement::Console(stmt) => self.visit_console(stmt),
            Statement::Decrement(stmt) => self.visit_decrement(stmt),
            Statement::Definition(stmt) => self.visit_definition(stmt),
            Statement::Finalize(stmt) => self.visit_finalize(stmt),
            Statement::Increment(stmt) => self.visit_increment(stmt),
            Statement::Iteration(stmt) => self.visit_iteration(stmt),
            Statement::Return(stmt) => self.visit_return(stmt),
        }
    }

    fn visit_return(&mut self, input: &'a ReturnStatement) -> String {
        match input.expression {
            // Skip empty return statements.
            Expression::Tuple(ref tuple) if tuple.elements.is_empty() => String::new(),
            _ => {
                let (operand, mut expression_instructions) = self.visit_expression(&input.expression);
                let instructions = operand
                    .split('\n')
                    .into_iter()
                    .zip(self.current_function.unwrap().output.iter())
                    .map(|(operand, output)| {
                        match output {
                            Output::Internal(output) => {
                                let visibility = if self.is_program_function {
                                    match self.in_finalize {
                                        // If in finalize block, the default visibility is public.
                                        true => match output.mode {
                                            Mode::None => Mode::Public,
                                            mode => mode,
                                        },
                                        // If not in finalize block, the default visibility is private.
                                        false => match output.mode {
                                            Mode::None => Mode::Private,
                                            mode => mode,
                                        },
                                    }
                                } else {
                                    // Only program functions have visibilities associated with their outputs.
                                    Mode::None
                                };
                                format!(
                                    "    output {} as {};\n",
                                    operand,
                                    self.visit_type_with_visibility(&output.type_, visibility)
                                )
                            }
                            Output::External(output) => {
                                format!(
                                    "    output {} as {}.aleo/{}.record;\n",
                                    operand, output.program_name, output.record,
                                )
                            }
                        }
                    })
                    .join("");

                expression_instructions.push_str(&instructions);

                expression_instructions
            }
        }
    }

    fn visit_definition(&mut self, _input: &'a DefinitionStatement) -> String {
        // TODO: If SSA is made optional, then conditionally enable codegen for DefinitionStatement
        // let (operand, expression_instructions) = self.visit_expression(&input.value);
        // self.variable_mapping.insert(&input.variable_name.name, operand);
        // expression_instructions
        unreachable!("DefinitionStatement's should not exist in SSA form.")
    }

    fn visit_increment(&mut self, input: &'a IncrementStatement) -> String {
        let (index, mut instructions) = self.visit_expression(&input.index);
        let (amount, amount_instructions) = self.visit_expression(&input.amount);
        instructions.push_str(&amount_instructions);
        instructions.push_str(&format!("    increment {}[{}] by {};\n", input.mapping, index, amount));

        instructions
    }

    fn visit_decrement(&mut self, input: &'a DecrementStatement) -> String {
        let (index, mut instructions) = self.visit_expression(&input.index);
        let (amount, amount_instructions) = self.visit_expression(&input.amount);
        instructions.push_str(&amount_instructions);
        instructions.push_str(&format!("    decrement {}[{}] by {};\n", input.mapping, index, amount));

        instructions
    }

    fn visit_finalize(&mut self, input: &'a FinalizeStatement) -> String {
        let mut instructions = String::new();
        let mut finalize_instruction = "    finalize".to_string();

        for argument in input.arguments.iter() {
            let (argument, argument_instructions) = self.visit_expression(argument);
            write!(finalize_instruction, " {}", argument).expect("failed to write to string");
            instructions.push_str(&argument_instructions);
        }
        writeln!(finalize_instruction, ";").expect("failed to write to string");

        finalize_instruction
    }

    fn visit_assign(&mut self, input: &'a AssignStatement) -> String {
        match &input.place {
            Expression::Identifier(identifier) => {
                let (operand, expression_instructions) = self.visit_expression(&input.value);
                self.variable_mapping.insert(&identifier.name, operand);
                expression_instructions
            }
            _ => unimplemented!(
                "Code generation for the left-hand side of an assignment is only implemented for `Identifier`s."
            ),
        }
    }

    fn visit_conditional(&mut self, _input: &'a ConditionalStatement) -> String {
        // TODO: Once SSA is made optional, create a Leo error informing the user to enable the SSA pass.
        unreachable!("`ConditionalStatement`s should not be in the AST at this phase of compilation.")
    }

    fn visit_iteration(&mut self, _input: &'a IterationStatement) -> String {
        // TODO: Once loop unrolling is made optional, create a Leo error informing the user to enable the loop unrolling pass..
        unreachable!("`IterationStatement`s should not be in the AST at this phase of compilation.");
    }

    fn visit_console(&mut self, input: &'a ConsoleStatement) -> String {
        let mut generate_assert_instruction = |name: &str, left: &'a Expression, right: &'a Expression| {
            let (left_operand, left_instructions) = self.visit_expression(left);
            let (right_operand, right_instructions) = self.visit_expression(right);
            let assert_instruction = format!("    {} {} {};\n", name, left_operand, right_operand);

            // Concatenate the instructions.
            let mut instructions = left_instructions;
            instructions.push_str(&right_instructions);
            instructions.push_str(&assert_instruction);

            instructions
        };
        match &input.function {
            ConsoleFunction::Assert(expr) => {
                let (operand, mut instructions) = self.visit_expression(expr);
                let assert_instruction = format!("    assert.eq {} true;\n", operand);

                instructions.push_str(&assert_instruction);
                instructions
            }
            ConsoleFunction::AssertEq(left, right) => generate_assert_instruction("assert.eq", left, right),
            ConsoleFunction::AssertNeq(left, right) => generate_assert_instruction("assert.neq", left, right),
        }
    }

    pub(crate) fn visit_block(&mut self, input: &'a Block) -> String {
        // For each statement in the block, visit it and add its instructions to the list.
        input.statements.iter().map(|stmt| self.visit_statement(stmt)).join("")
    }
}
