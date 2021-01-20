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

//! Enforces a circuit expression in a compiled Leo program.

use crate::{
    errors::ExpressionError,
    program::{ConstrainedProgram},
    value::{ConstrainedCircuitMember, ConstrainedValue},
    GroupType,
};
use leo_asg::{CircuitInitExpression, CircuitMemberBody, Span};

use snarkvm_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn enforce_circuit<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: &str,
        function_scope: &str,
        expr: &CircuitInitExpression,
        span: &Span,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {
        // Circuit definitions are located at the minimum file scope
        let minimum_scope = file_scope.split('_').next().unwrap();

        let circuit = expr.circuit.body.borrow().upgrade().expect("circuit init stale circuit ref");
        let members = circuit.members.borrow();
        let circuit_identifier = expr.circuit.name.borrow().clone();

        let mut resolved_members = Vec::with_capacity(members.len());

        // type checking is already done in asg
        for (name, inner) in expr.values.iter() {
            let target = members.get(&name.name).expect("illegal name in asg circuit init expression");
            match target {
                CircuitMemberBody::Variable(_type_) => {
                    let variable_value = self.enforce_expression(
                        cs,
                        file_scope,
                        function_scope,
                        inner,
                    )?;
                    resolved_members.push(ConstrainedCircuitMember(name.clone(), variable_value));
                }
                _ => return Err(ExpressionError::expected_circuit_member(name.to_string(), span.clone()))
            }
        }

        let id = uuid::Uuid::new_v4();
        let value = ConstrainedValue::CircuitExpression(
            circuit,
            id.clone(),
            resolved_members,
        );
        self.store(id, value.clone());
        Ok(value)
    }
}
