// Copyright (C) 2019-2023 Aleo Systems Inc.
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

use crate::{TypeChecker, VariableType};

use leo_ast::{ExpressionVisitor, Instruction, InstructionVisitor, IntegerType, Node, Opcode, Type};
use leo_errors::TypeCheckerError;

use itertools::Itertools;

impl<'a> TypeChecker<'a> {
    // Helper to check that the structure of the instruction is well-formed.
    fn check_instruction_is_well_formed<const NUM_OPERANDS: usize, const NUM_DESTINATIONS: usize>(
        &mut self,
        instruction: &'a Instruction,
    ) -> bool {
        // A flag to indicate if the instruction is well-formed.
        let mut is_well_formed = true;
        // Check that the number of operands is NUM_OPERANDS.
        if instruction.operands.len() != NUM_OPERANDS {
            is_well_formed = false;
            self.emit_err(TypeCheckerError::malformed_instruction(
                format!("Expected {NUM_OPERANDS} operands."),
                instruction.span,
            ));
        };
        // Check that the number of destinations is NUM_DESTINATIONS.
        if instruction.destinations.len() != NUM_DESTINATIONS {
            is_well_formed = false;
            self.emit_err(TypeCheckerError::malformed_instruction(
                format!("Expected {NUM_DESTINATIONS} destination registers."),
                instruction.span,
            ));
        };
        is_well_formed
    }

    // Helper to type check standard instructions.
    fn check_instruction<const NUM_OPERANDS: usize, const NUM_DESTINATIONS: usize>(
        &mut self,
        instruction: &'a Instruction,
        expected_types: &[([Type; NUM_OPERANDS], [Type; NUM_DESTINATIONS])],
    ) {
        // Check that the types of the operands match one of the expected operand types.
        println!("NUM_OPERANDS: {}", NUM_OPERANDS);
        println!("NUM_DESTINATIONS: {}", NUM_DESTINATIONS);
        println!("instruction: {:?}", instruction);
        // Check that the structure of the instruction is well-formed.
        if self.check_instruction_is_well_formed::<NUM_OPERANDS, NUM_DESTINATIONS>(instruction) {
            // Get the types of the operands.
            let operand_types = instruction
                .operands
                .iter()
                .map(|operand| self.visit_expression(operand, &None))
                .collect_vec();

            println!("operand_types: {:?}", operand_types);

            let destination_types = expected_types
                .iter()
                .find(|(expected_operand_types, _)| {
                    operand_types
                        .iter()
                        .zip_eq(expected_operand_types.iter())
                        .all(|(operand_type, expected_type)| match operand_type {
                            Some(operand_type) => operand_type.eq_flat(expected_type),
                            None => false,
                        })
                })
                .map(|(_, expected_destination_types)| expected_destination_types);
            // If the destination types are found, add the destination registers to the symbol table.
            // Otherwise, emit an error.
            match destination_types {
                Some(destination_types) => {
                    for (destination, destination_type) in
                        instruction.destinations.iter().zip_eq(destination_types.iter())
                    {
                        self.insert_variable(
                            destination.name,
                            destination_type.clone(),
                            destination.span,
                            VariableType::Mut,
                        );
                    }
                }
                None => self.emit_err(TypeCheckerError::invalid_instruction_operand_types(
                    &instruction.opcode,
                    expected_types
                        .iter()
                        .map(|(operand_types, _)| format!("({})", operand_types.iter().join(", ")))
                        .join(", "),
                    instruction.span,
                )),
            }
        }
    }

    // Helper to type check commit instructions.
    fn check_commit_instruction(&mut self, instruction: &'a Instruction, output_type: Type) {
        // Check that the structure of the instruction is well-formed.
        if self.check_instruction_is_well_formed::<2, 1>(instruction) {
            // Check that the second operand is a scalar.
            let second_type = self.visit_expression(&instruction.operands[1], &None);
            self.assert_type(&second_type, &Type::Scalar, instruction.operands[1].span());
            // Add the destination register to the symbol table.
            let destination = &instruction.destinations[0];
            self.insert_variable(destination.name, output_type, destination.span, VariableType::Mut);
        }
    }
}

macro_rules! declare_types {
    ($(
        (
            $($operand: expr),* => $($destination: expr),*
        )
    ),*) => {
        &[
            $(([$($operand),*], [$($destination),*])),*
        ]
    };
}

impl<'a> InstructionVisitor<'a> for TypeChecker<'a> {
    fn visit_instruction(&mut self, instruction: &'a Instruction) {
        match instruction.opcode {
            Opcode::Abs | Opcode::AbsWrapped => self.check_instruction(instruction, declare_types!(
                (Type::Integer(IntegerType::I8) => Type::Integer(IntegerType::I8)),
                (Type::Integer(IntegerType::I16) => Type::Integer(IntegerType::I16)),
                (Type::Integer(IntegerType::I32) => Type::Integer(IntegerType::I32)),
                (Type::Integer(IntegerType::I64) => Type::Integer(IntegerType::I64)),
                (Type::Integer(IntegerType::I128) => Type::Integer(IntegerType::I128))
            )),
            Opcode::Add | Opcode::Div | Opcode::Sub => self.check_instruction(instruction, declare_types!(
                (Type::Field, Type::Field => Type::Field),
                (Type::Integer(IntegerType::I8), Type::Integer(IntegerType::I8) => Type::Integer(IntegerType::I8)),
                (Type::Integer(IntegerType::I16), Type::Integer(IntegerType::I16) => Type::Integer(IntegerType::I16)),
                (Type::Integer(IntegerType::I32), Type::Integer(IntegerType::I32) => Type::Integer(IntegerType::I32)),
                (Type::Integer(IntegerType::I64), Type::Integer(IntegerType::I64) => Type::Integer(IntegerType::I64)),
                (Type::Integer(IntegerType::I128), Type::Integer(IntegerType::I128) => Type::Integer(IntegerType::I128)),
                (Type::Integer(IntegerType::U8), Type::Integer(IntegerType::U8) => Type::Integer(IntegerType::U8)),
                (Type::Integer(IntegerType::U16), Type::Integer(IntegerType::U16) => Type::Integer(IntegerType::U16)),
                (Type::Integer(IntegerType::U32), Type::Integer(IntegerType::U32) => Type::Integer(IntegerType::U32)),
                (Type::Integer(IntegerType::U64), Type::Integer(IntegerType::U64) => Type::Integer(IntegerType::U64)),
                (Type::Integer(IntegerType::U128), Type::Integer(IntegerType::U128) => Type::Integer(IntegerType::U128))
            )),
            Opcode::AddWrapped
            | Opcode::DivWrapped
            | Opcode::MulWrapped
            | Opcode::Rem
            | Opcode::RemWrapped
            | Opcode::SubWrapped => self.check_instruction(instruction, declare_types!(
                (Type::Integer(IntegerType::I8), Type::Integer(IntegerType::I8) => Type::Integer(IntegerType::I8)),
                (Type::Integer(IntegerType::I16), Type::Integer(IntegerType::I16) => Type::Integer(IntegerType::I16)),
                (Type::Integer(IntegerType::I32), Type::Integer(IntegerType::I32) => Type::Integer(IntegerType::I32)),
                (Type::Integer(IntegerType::I64), Type::Integer(IntegerType::I64) => Type::Integer(IntegerType::I64)),
                (Type::Integer(IntegerType::I128), Type::Integer(IntegerType::I128) => Type::Integer(IntegerType::I128)),
                (Type::Integer(IntegerType::U8), Type::Integer(IntegerType::U8) => Type::Integer(IntegerType::U8)),
                (Type::Integer(IntegerType::U16), Type::Integer(IntegerType::U16) => Type::Integer(IntegerType::U16)),
                (Type::Integer(IntegerType::U32), Type::Integer(IntegerType::U32) => Type::Integer(IntegerType::U32)),
                (Type::Integer(IntegerType::U64), Type::Integer(IntegerType::U64) => Type::Integer(IntegerType::U64)),
                (Type::Integer(IntegerType::U128), Type::Integer(IntegerType::U128) => Type::Integer(IntegerType::U128))
            )),
            Opcode::AssertEq | Opcode::AssertNeq => {
                // Check that the instruction is well-formed.
                if self.check_instruction_is_well_formed::<2, 0>(instruction) {
                    // Check that the operands are the same type.
                    let lhs = self.visit_expression(&instruction.operands[0], &None);
                    let rhs = self.visit_expression(&instruction.operands[1], &None);
                    self.check_eq_types(&lhs, &rhs, instruction.span);
                }
            }
            Opcode::CommitBHP256 | Opcode::CommitBHP512 | Opcode::CommitBHP768 | Opcode::CommitBHP1024 => self.check_commit_instruction(instruction, Type::Field),
            Opcode::CommitPED64 | Opcode::CommitPED128 => self.check_commit_instruction(instruction, Type::Group),
            Opcode::Double => self.check_instruction(instruction, declare_types!(
                (Type::Field => Type::Field),
                (Type::Group => Type::Group)
            )),
            Opcode::GreaterThan
            | Opcode::GreaterThanOrEqual
            | Opcode::LessThan
            | Opcode::LessThanOrEqual => self.check_instruction(instruction, declare_types!(
                (Type::Field, Type::Field => Type::Boolean),
                (Type::Integer(IntegerType::I8), Type::Integer(IntegerType::I8) => Type::Boolean),
                (Type::Integer(IntegerType::I16), Type::Integer(IntegerType::I16) => Type::Boolean),
                (Type::Integer(IntegerType::I32), Type::Integer(IntegerType::I32) => Type::Boolean),
                (Type::Integer(IntegerType::I64), Type::Integer(IntegerType::I64) => Type::Boolean),
                (Type::Integer(IntegerType::I128), Type::Integer(IntegerType::I128) => Type::Boolean),
                (Type::Integer(IntegerType::U8), Type::Integer(IntegerType::U8) => Type::Boolean),
                (Type::Integer(IntegerType::U16), Type::Integer(IntegerType::U16) => Type::Boolean),
                (Type::Integer(IntegerType::U32), Type::Integer(IntegerType::U32) => Type::Boolean),
                (Type::Integer(IntegerType::U64), Type::Integer(IntegerType::U64) => Type::Boolean),
                (Type::Integer(IntegerType::U128), Type::Integer(IntegerType::U128) => Type::Boolean),
                (Type::Scalar, Type::Scalar => Type::Boolean)
            )),
            Opcode::HashBHP256
            | Opcode::HashBHP512
            | Opcode::HashBHP768
            | Opcode::HashBHP1024
            | Opcode::HashPED64
            | Opcode::HashPED128
            | Opcode::HashPSD2
            | Opcode::HashPSD4
            | Opcode::HashPSD8 => {
                // Check that the instruction is well-formed.
                if self.check_instruction_is_well_formed::<1, 1>(instruction) {
                    // Add the destination to the symbol table.
                    let destination = &instruction.destinations[0];
                    self.insert_variable(destination.name, Type::Boolean, destination.span, VariableType::Mut);
                }
            }
            Opcode::Inv
            | Opcode::Square
            | Opcode::SquareRoot => self.check_instruction(instruction, declare_types!(
                (Type::Field => Type::Field)
            )),
            Opcode::IsEq | Opcode::IsNeq => {
                // Check that the instruction is well formed.
                if self.check_instruction_is_well_formed::<2, 1>(instruction) {
                    // Check that the operands are of the same type.
                    let lhs = self.visit_expression(&instruction.operands[0], &None);
                    let rhs = self.visit_expression(&instruction.operands[1], &None);
                    self.check_eq_types(&lhs, &rhs, instruction.span);
                    // Add the destination to the symbol table.
                    let destination = &instruction.destinations[0];
                    self.insert_variable(destination.name, Type::Boolean, destination.span, VariableType::Mut);
                }
            }
            Opcode::Modulo => self.check_instruction(instruction, declare_types!(
                (Type::Integer(IntegerType::U8), Type::Integer(IntegerType::U8) => Type::Integer(IntegerType::U8)),
                (Type::Integer(IntegerType::U16), Type::Integer(IntegerType::U16) => Type::Integer(IntegerType::U16)),
                (Type::Integer(IntegerType::U32), Type::Integer(IntegerType::U32) => Type::Integer(IntegerType::U32)),
                (Type::Integer(IntegerType::U64), Type::Integer(IntegerType::U64) => Type::Integer(IntegerType::U64)),
                (Type::Integer(IntegerType::U128), Type::Integer(IntegerType::U128) => Type::Integer(IntegerType::U128))
            )),
            Opcode::Mul => self.check_instruction(instruction, declare_types!(
                (Type::Field, Type::Field => Type::Field),
                (Type::Group, Type::Scalar => Type::Group),
                (Type::Scalar, Type::Group => Type::Group),
                (Type::Integer(IntegerType::I8), Type::Integer(IntegerType::I8) => Type::Integer(IntegerType::I8)),
                (Type::Integer(IntegerType::I16), Type::Integer(IntegerType::I16) => Type::Integer(IntegerType::I16)),
                (Type::Integer(IntegerType::I32), Type::Integer(IntegerType::I32) => Type::Integer(IntegerType::I32)),
                (Type::Integer(IntegerType::I64), Type::Integer(IntegerType::I64) => Type::Integer(IntegerType::I64)),
                (Type::Integer(IntegerType::I128), Type::Integer(IntegerType::I128) => Type::Integer(IntegerType::I128)),
                (Type::Integer(IntegerType::U8), Type::Integer(IntegerType::U8) => Type::Integer(IntegerType::U8)),
                (Type::Integer(IntegerType::U16), Type::Integer(IntegerType::U16) => Type::Integer(IntegerType::U16)),
                (Type::Integer(IntegerType::U32), Type::Integer(IntegerType::U32) => Type::Integer(IntegerType::U32)),
                (Type::Integer(IntegerType::U64), Type::Integer(IntegerType::U64) => Type::Integer(IntegerType::U64)),
                (Type::Integer(IntegerType::U128), Type::Integer(IntegerType::U128) => Type::Integer(IntegerType::U128))
            )),
            Opcode::Nand | Opcode::Nor => self.check_instruction(instruction, declare_types!(
                (Type::Boolean, Type::Boolean => Type::Boolean)
            )),
            Opcode::Neg => self.check_instruction(instruction, declare_types!(
                (Type::Field => Type::Field),
                (Type::Group => Type::Group),
                (Type::Integer(IntegerType::I8) => Type::Integer(IntegerType::I8)),
                (Type::Integer(IntegerType::I16) => Type::Integer(IntegerType::I16)),
                (Type::Integer(IntegerType::I32) => Type::Integer(IntegerType::I32)),
                (Type::Integer(IntegerType::I64) => Type::Integer(IntegerType::I64)),
                (Type::Integer(IntegerType::I128) => Type::Integer(IntegerType::I128))
            )),
            Opcode::And | Opcode::Not | Opcode::Or | Opcode::Xor  => self.check_instruction(instruction, declare_types!(
                (Type::Boolean, Type::Boolean => Type::Boolean),
                (Type::Integer(IntegerType::I8), Type::Integer(IntegerType::I8) => Type::Integer(IntegerType::I8)),
                (Type::Integer(IntegerType::I16), Type::Integer(IntegerType::I16) => Type::Integer(IntegerType::I16)),
                (Type::Integer(IntegerType::I32), Type::Integer(IntegerType::I32) => Type::Integer(IntegerType::I32)),
                (Type::Integer(IntegerType::I64), Type::Integer(IntegerType::I64) => Type::Integer(IntegerType::I64)),
                (Type::Integer(IntegerType::I128), Type::Integer(IntegerType::I128) => Type::Integer(IntegerType::I128)),
                (Type::Integer(IntegerType::U8), Type::Integer(IntegerType::U8) => Type::Integer(IntegerType::U8)),
                (Type::Integer(IntegerType::U16), Type::Integer(IntegerType::U16) => Type::Integer(IntegerType::U16)),
                (Type::Integer(IntegerType::U32), Type::Integer(IntegerType::U32) => Type::Integer(IntegerType::U32)),
                (Type::Integer(IntegerType::U64), Type::Integer(IntegerType::U64) => Type::Integer(IntegerType::U64)),
                (Type::Integer(IntegerType::U128), Type::Integer(IntegerType::U128) => Type::Integer(IntegerType::U128))
            )),
            Opcode::Pow
            | Opcode::PowWrapped
            | Opcode::Shl
            | Opcode::ShlWrapped
            | Opcode::Shr
            | Opcode::ShrWrapped => self.check_instruction(instruction, declare_types!(
                (Type::Integer(IntegerType::I8), Type::Integer(IntegerType::U8) => Type::Integer(IntegerType::I8)),
                (Type::Integer(IntegerType::I8), Type::Integer(IntegerType::U16) => Type::Integer(IntegerType::I8)),
                (Type::Integer(IntegerType::I8), Type::Integer(IntegerType::U32) => Type::Integer(IntegerType::I8)),
                (Type::Integer(IntegerType::I16), Type::Integer(IntegerType::U8) => Type::Integer(IntegerType::I16)),
                (Type::Integer(IntegerType::I16), Type::Integer(IntegerType::U16) => Type::Integer(IntegerType::I16)),
                (Type::Integer(IntegerType::I16), Type::Integer(IntegerType::U32) => Type::Integer(IntegerType::I16)),
                (Type::Integer(IntegerType::I32), Type::Integer(IntegerType::U8) => Type::Integer(IntegerType::I32)),
                (Type::Integer(IntegerType::I32), Type::Integer(IntegerType::U16) => Type::Integer(IntegerType::I32)),
                (Type::Integer(IntegerType::I32), Type::Integer(IntegerType::U32) => Type::Integer(IntegerType::I32)),
                (Type::Integer(IntegerType::I64), Type::Integer(IntegerType::U8) => Type::Integer(IntegerType::I64)),
                (Type::Integer(IntegerType::I64), Type::Integer(IntegerType::U16) => Type::Integer(IntegerType::I64)),
                (Type::Integer(IntegerType::I64), Type::Integer(IntegerType::U32) => Type::Integer(IntegerType::I64)),
                (Type::Integer(IntegerType::I128), Type::Integer(IntegerType::U8) => Type::Integer(IntegerType::I128)),
                (Type::Integer(IntegerType::I128), Type::Integer(IntegerType::U16) => Type::Integer(IntegerType::I128)),
                (Type::Integer(IntegerType::I128), Type::Integer(IntegerType::U32) => Type::Integer(IntegerType::I128)),
                (Type::Integer(IntegerType::U8), Type::Integer(IntegerType::U8) => Type::Integer(IntegerType::U8)),
                (Type::Integer(IntegerType::U8), Type::Integer(IntegerType::U16) => Type::Integer(IntegerType::U8)),
                (Type::Integer(IntegerType::U8), Type::Integer(IntegerType::U32) => Type::Integer(IntegerType::U8)),
                (Type::Integer(IntegerType::U16), Type::Integer(IntegerType::U8) => Type::Integer(IntegerType::U16)),
                (Type::Integer(IntegerType::U16), Type::Integer(IntegerType::U16) => Type::Integer(IntegerType::U16)),
                (Type::Integer(IntegerType::U16), Type::Integer(IntegerType::U32) => Type::Integer(IntegerType::U16)),
                (Type::Integer(IntegerType::U32), Type::Integer(IntegerType::U8) => Type::Integer(IntegerType::U32)),
                (Type::Integer(IntegerType::U32), Type::Integer(IntegerType::U16) => Type::Integer(IntegerType::U32)),
                (Type::Integer(IntegerType::U32), Type::Integer(IntegerType::U32) => Type::Integer(IntegerType::U32)),
                (Type::Integer(IntegerType::U64), Type::Integer(IntegerType::U8) => Type::Integer(IntegerType::U64)),
                (Type::Integer(IntegerType::U64), Type::Integer(IntegerType::U16) => Type::Integer(IntegerType::U64)),
                (Type::Integer(IntegerType::U64), Type::Integer(IntegerType::U32) => Type::Integer(IntegerType::U64)),
                (Type::Integer(IntegerType::U128), Type::Integer(IntegerType::U8) => Type::Integer(IntegerType::U128)),
                (Type::Integer(IntegerType::U128), Type::Integer(IntegerType::U16) => Type::Integer(IntegerType::U128)),
                (Type::Integer(IntegerType::U128), Type::Integer(IntegerType::U32) => Type::Integer(IntegerType::U128))
            )),
            Opcode::Ternary => self.check_instruction(instruction, declare_types!(
                (Type::Boolean, Type::Boolean, Type::Boolean => Type::Boolean),
                (Type::Boolean, Type::Field, Type::Field => Type::Field),
                (Type::Boolean, Type::Group, Type::Group => Type::Group),
                (Type::Boolean, Type::Integer(IntegerType::I8), Type::Integer(IntegerType::I8) => Type::Integer(IntegerType::I8)),
                (Type::Boolean, Type::Integer(IntegerType::I16), Type::Integer(IntegerType::I16) => Type::Integer(IntegerType::I16)),
                (Type::Boolean, Type::Integer(IntegerType::I32), Type::Integer(IntegerType::I32) => Type::Integer(IntegerType::I32)),
                (Type::Boolean, Type::Integer(IntegerType::I64), Type::Integer(IntegerType::I64) => Type::Integer(IntegerType::I64)),
                (Type::Boolean, Type::Integer(IntegerType::I128), Type::Integer(IntegerType::I128) => Type::Integer(IntegerType::I128)),
                (Type::Boolean, Type::Integer(IntegerType::U8), Type::Integer(IntegerType::U8) => Type::Integer(IntegerType::U8)),
                (Type::Boolean, Type::Integer(IntegerType::U16), Type::Integer(IntegerType::U16) => Type::Integer(IntegerType::U16)),
                (Type::Boolean, Type::Integer(IntegerType::U32), Type::Integer(IntegerType::U32) => Type::Integer(IntegerType::U32)),
                (Type::Boolean, Type::Integer(IntegerType::U64), Type::Integer(IntegerType::U64) => Type::Integer(IntegerType::U64)),
                (Type::Boolean, Type::Integer(IntegerType::U128), Type::Integer(IntegerType::U128) => Type::Integer(IntegerType::U128)),
                (Type::Boolean, Type::Scalar, Type::Scalar => Type::Scalar)
            )),
        }
    }
}
