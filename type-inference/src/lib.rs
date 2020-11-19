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

//! A type inference check for a Leo program.
//!
//! This module contains the [`TypeInference`] type, which stores information needed to run a type
//! inference check over a program.
//!
//! A new [`TypeInference`] type can be created from a [`LeoAst`] type and a [`Symbol Table`].

#[macro_use]
extern crate thiserror;

pub mod assertions;
pub use self::assertions::*;

pub mod type_inference;
pub use self::type_inference::*;

pub mod errors;
pub use self::errors::*;

pub mod objects;
pub use self::objects::*;
