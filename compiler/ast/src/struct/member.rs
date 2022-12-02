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

use crate::{Identifier, Node, Type};
use leo_span::{Span, Symbol};

use serde::{Deserialize, Serialize};
use std::fmt;

/// A member of a struct definition, e.g `foobar: u8`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructMember {
    /// The identifier of the member.
    pub identifier: Identifier,
    /// The type of the member.
    pub type_: Type,
    /// The span of the member.
    pub span: Span,
}

impl StructMember {
    /// Returns the name of the struct member without span.
    pub fn name(&self) -> Symbol {
        self.identifier.name
    }
}

impl fmt::Display for StructMember {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.identifier, self.type_)
    }
}

crate::simple_node_impl!(StructMember);
