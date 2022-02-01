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

use crate::Identifier;
use leo_span::{sym, Span, Symbol};

use serde::{Deserialize, Serialize};
use std::fmt;

/// An annotation `@name (arguments)?`.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Annotation {
    /// The span including `name ( arguments )`.
    pub span: Span,
    /// The name of the annotation.
    pub name: Identifier,
    /// Arguments for the annotation, if any.
    pub arguments: Vec<Symbol>,
}

/// The set of allowed annotations.
const ALLOWED_ANNOTATIONS: &[Symbol] = &[sym::test];

impl Annotation {
    /// Is the annotation valid?
    pub fn is_valid_annotation(&self) -> bool {
        ALLOWED_ANNOTATIONS.iter().any(|name| self.name.name == *name)
    }
}

impl fmt::Display for Annotation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "@{:}(", self.name)?;
        for arg in &self.arguments {
            write!(f, "{:},", arg)?;
        }
        write!(f, ")")
    }
}
