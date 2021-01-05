// Copyright (C) 2019-2020 Aleo Systems Inc.
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

//! Enforces an tuple expression in a compiled Leo program.

use crate::{errors::ExpressionError, program::ConstrainedProgram, value::ConstrainedValue, GroupType};
use leo_ast::{Expression, Span, Type};

use snarkvm_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    /// Enforce tuple expressions
    pub fn enforce_tuple<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: &str,
        function_scope: &str,
        expected_type: Option<Type>,
        tuple: Vec<Expression>,
        span: Span,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {
        // Check explicit tuple type dimension if given
        let mut expected_types = vec![];

        match expected_type {
            Some(Type::Tuple(ref types)) => {
                expected_types = types.clone();
            }
            Some(ref type_) => {
                return Err(ExpressionError::unexpected_tuple(
                    type_.to_string(),
                    format!("{:?}", tuple),
                    span,
                ));
            }
            None => {}
        }

        let mut result = Vec::with_capacity(tuple.len());
        for (i, expression) in tuple.into_iter().enumerate() {
            let type_ = if expected_types.is_empty() {
                None
            } else {
                Some(expected_types[i].clone())
            };

            result.push(self.enforce_expression(cs, file_scope, function_scope, type_, expression)?);
        }

        Ok(ConstrainedValue::Tuple(result))
    }
}
