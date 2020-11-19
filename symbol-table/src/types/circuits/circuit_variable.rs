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

use crate::{Attribute, CircuitType, Type};
use leo_ast::Identifier;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CircuitVariableType {
    /// The name of the circuit variable
    pub identifier: Identifier,
    /// The type of the circuit variable
    pub type_: Type,
    /// The attribute of the circuit variable
    pub attribute: Option<Attribute>,
}

impl From<&CircuitType> for CircuitVariableType {
    fn from(type_: &CircuitType) -> Self {
        Self {
            identifier: type_.identifier.clone(),
            type_: Type::Circuit(type_.identifier.clone()),
            attribute: None,
        }
    }
}
