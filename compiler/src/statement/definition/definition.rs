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

//! Enforces a definition statement in a compiled Leo program.

use crate::{errors::StatementError, program::ConstrainedProgram, ConstrainedValue, GroupType};
use leo_asg::{DefinitionStatement, Span, Variable};

use snarkvm_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    fn enforce_single_definition<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        function_scope: &str,
        variable: &Variable,
        mut value: ConstrainedValue<F, G>,
        span: &Span,
    ) -> Result<(), StatementError> {

        self.store_definition(function_scope, variable, value);

        Ok(())
    }

    fn enforce_multiple_definition<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        function_scope: &str,
        variable_names: &Vec<Variable>,
        values: Vec<ConstrainedValue<F, G>>,
        span: &Span,
    ) -> Result<(), StatementError> {
        if values.len() != variable_names.len() {
            return Err(StatementError::invalid_number_of_definitions(
                values.len(),
                variable_names.len(),
                span.to_owned(),
            ));
        }

        for (variable, value) in variable_names.into_iter().zip(values.into_iter()) {
            self.enforce_single_definition(cs, function_scope, variable, value, span)?;
        }

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn enforce_definition_statement<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: &str,
        function_scope: &str,
        statement: &DefinitionStatement,
    ) -> Result<(), StatementError> {
        let num_variables = statement.variables.len();
        let expression =
            self.enforce_expression(cs, file_scope, function_scope, &statement.value)?;

        let span = statement.span.clone().unwrap_or_default();
        if num_variables == 1 {
            // Define a single variable with a single value
            self.enforce_single_definition(cs, function_scope, statement.variables.get(0).unwrap(), expression, &span)
        } else {
            // Define multiple variables for an expression that returns multiple results (multiple definition)
            let values = match expression {
                // ConstrainedValue::Return(values) => values,
                ConstrainedValue::Tuple(values) => values,
                value => return Err(StatementError::multiple_definition(value.to_string(), span.clone())),
            };

            self.enforce_multiple_definition(
                cs,
                function_scope,
                &statement.variables,
                values,
                &span,
            )
        }
    }
}
