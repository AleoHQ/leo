// Copyright (C) 2019-2021 Aleo Systems Inc.
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

use std::cell::Cell;

use crate::program::Program;
use leo_asg::Expression;
use leo_errors::Result;
use snarkvm_ir::Value;

impl<'a> Program<'a> {
    /// Enforce tuple expressions
    pub fn enforce_tuple(&mut self, tuple: &[Cell<&'a Expression<'a>>]) -> Result<Value> {
        let mut result = Vec::with_capacity(tuple.len());
        for expression in tuple.iter() {
            result.push(self.enforce_expression(expression.get())?);
        }

        Ok(Value::Tuple(result))
    }
}
