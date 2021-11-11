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

//! The parser to convert Leo code text into an [`AST`] type.
//!
//! This module contains the [`parse_ast()`] method which calls the underlying [`parse()`]
//! method to create a new program ast.

#![allow(clippy::vec_init_then_push)]
#![doc = include_str!("../README.md")]

pub(crate) mod tokenizer;
use leo_input::LeoInputParser;
pub use tokenizer::KEYWORD_TOKENS;
pub(crate) use tokenizer::*;

pub mod parser;
pub use parser::*;

use leo_ast::{Ast, Input};
use leo_errors::emitter::Handler;
use leo_errors::Result;

#[cfg(test)]
mod test;

/// Creates a new AST from a given file path and source code text.
pub fn parse_ast<T: AsRef<str>, Y: AsRef<str>>(handler: &Handler, path: T, source: Y) -> Result<Ast> {
    Ok(Ast::new(parser::parse(handler, path.as_ref(), source.as_ref())?))
}

/// Parses program input from from the input file path and state file path
pub fn parse_program_input<T: AsRef<str>, Y: AsRef<str>, T2: AsRef<str>, Y2: AsRef<str>>(
    input_string: T,
    input_path: Y,
    state_string: T2,
    state_path: Y2,
) -> Result<Input> {
    let input_syntax_tree = LeoInputParser::parse_file(input_string.as_ref()).map_err(|mut e| {
        e.set_path(
            input_path.as_ref(),
            &input_string
                .as_ref()
                .lines()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()[..],
        );

        e
    })?;
    let state_syntax_tree = LeoInputParser::parse_file(state_string.as_ref()).map_err(|mut e| {
        e.set_path(
            state_path.as_ref(),
            &state_string
                .as_ref()
                .lines()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()[..],
        );

        e
    })?;

    let mut input = Input::new();
    input.parse_input(input_syntax_tree).map_err(|mut e| {
        e.set_path(
            input_path.as_ref(),
            &input_string
                .as_ref()
                .lines()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()[..],
        );

        e
    })?;
    input.parse_state(state_syntax_tree).map_err(|mut e| {
        e.set_path(
            state_path.as_ref(),
            &state_string
                .as_ref()
                .lines()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()[..],
        );

        e
    })?;

    Ok(input)
}
