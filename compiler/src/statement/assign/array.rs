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

//! Enforces an array assignment statement in a compiled Leo program.

use crate::{errors::StatementError, program::ConstrainedProgram, value::ConstrainedValue, GroupType};
use leo_typed::{RangeOrExpression, Span};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::{boolean::Boolean, select::CondSelectGadget},
    },
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    #[allow(clippy::too_many_arguments)]
    pub fn assign_array<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: &str,
        function_scope: &str,
        indicator: Option<Boolean>,
        name: &str,
        range_or_expression: RangeOrExpression,
        mut new_value: ConstrainedValue<F, G>,
        span: &Span,
    ) -> Result<(), StatementError> {
        let condition = indicator.unwrap_or(Boolean::Constant(true));

        // Resolve index so we know if we are assigning to a single value or a range of values
        match range_or_expression {
            RangeOrExpression::Expression(index) => {
                let index = self.enforce_index(cs, file_scope, function_scope, index, span)?;

                // Modify the single value of the array in place
                match self.get_mutable_assignee(name, &span)? {
                    ConstrainedValue::Array(old) => {
                        new_value.resolve_type(Some(old[index].to_type(&span)?), &span)?;

                        let name_unique = format!("select {} {}:{}", new_value, span.line, span.start);
                        let selected_value = ConstrainedValue::conditionally_select(
                            cs.ns(|| name_unique),
                            &condition,
                            &new_value,
                            &old[index],
                        )
                        .map_err(|_| {
                            StatementError::select_fail(new_value.to_string(), old[index].to_string(), span.to_owned())
                        })?;

                        old[index] = selected_value;
                    }
                    _ => return Err(StatementError::array_assign_index(span.to_owned())),
                }
            }
            RangeOrExpression::Range(from, to) => {
                let from_index = match from {
                    Some(integer) => self.enforce_index(cs, file_scope, function_scope, integer, span)?,
                    None => 0usize,
                };
                let to_index_option = match to {
                    Some(integer) => Some(self.enforce_index(cs, file_scope, function_scope, integer, span)?),
                    None => None,
                };

                // Modify the range of values of the array
                let old_array = self.get_mutable_assignee(name, &span)?;
                let new_array = match (old_array.clone(), new_value) {
                    (ConstrainedValue::Array(mut mutable), ConstrainedValue::Array(new)) => {
                        let to_index = to_index_option.unwrap_or(mutable.len());

                        mutable.splice(from_index..to_index, new.iter().cloned());
                        ConstrainedValue::Array(mutable)
                    }
                    _ => return Err(StatementError::array_assign_range(span.to_owned())),
                };
                let name_unique = format!("select {} {}:{}", new_array, span.line, span.start);
                let selected_array =
                    ConstrainedValue::conditionally_select(cs.ns(|| name_unique), &condition, &new_array, old_array)
                        .map_err(|_| {
                            StatementError::select_fail(new_array.to_string(), old_array.to_string(), span.to_owned())
                        })?;

                *old_array = selected_array;
            }
        }

        Ok(())
    }
}
