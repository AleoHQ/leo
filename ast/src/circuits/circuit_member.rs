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

use crate::{Function, Identifier, Type};
use leo_grammar::circuits::{
    CircuitFunction as GrammarCircuitFunction,
    CircuitMember as GrammarCircuitMember,
    CircuitVariableDefinition as GrammarCircuitVariableDefinition,
};

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CircuitMember {
    // (is_mutable, variable_name, variable_type)
    CircuitVariable(bool, Identifier, Type),
    // (is_static, function)
    CircuitFunction(bool, Function),
}

impl<'ast> From<GrammarCircuitVariableDefinition<'ast>> for CircuitMember {
    fn from(circuit_value: GrammarCircuitVariableDefinition<'ast>) -> Self {
        CircuitMember::CircuitVariable(
            circuit_value.mutable.is_some(),
            Identifier::from(circuit_value.identifier),
            Type::from(circuit_value.type_),
        )
    }
}

impl<'ast> From<GrammarCircuitFunction<'ast>> for CircuitMember {
    fn from(circuit_function: GrammarCircuitFunction<'ast>) -> Self {
        CircuitMember::CircuitFunction(
            circuit_function._static.is_some(),
            Function::from(circuit_function.function),
        )
    }
}

impl<'ast> From<GrammarCircuitMember<'ast>> for CircuitMember {
    fn from(object: GrammarCircuitMember<'ast>) -> Self {
        match object {
            GrammarCircuitMember::CircuitVariableDefinition(circuit_value) => CircuitMember::from(circuit_value),
            GrammarCircuitMember::CircuitFunction(circuit_function) => CircuitMember::from(circuit_function),
        }
    }
}

impl fmt::Display for CircuitMember {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CircuitMember::CircuitVariable(ref mutable, ref identifier, ref type_) => {
                if *mutable {
                    write!(f, "mut ")?;
                }
                write!(f, "{}: {}", identifier, type_)
            }
            CircuitMember::CircuitFunction(ref static_, ref function) => {
                if *static_ {
                    write!(f, "static ")?;
                }
                write!(f, "{}", function)
            }
        }
    }
}
