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

use crate::{assert_satisfied, expect_compiler_error, generate_main_input, parse_program};
use leo_typed::InputValue;

#[test]
fn test_print() {
    let bytes = include_bytes!("print.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_print_fail() {
    let bytes = include_bytes!("print_fail.leo");

    assert!(parse_program(bytes).is_err());
}

#[test]
fn test_print_parameter() {
    let bytes = include_bytes!("print_parameter.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_print_parameter_many() {
    let bytes = include_bytes!("print_parameter_many.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_print_parameter_fail_unknown() {
    let bytes = include_bytes!("print_parameter_fail_unknown.leo");
    let program = parse_program(bytes).unwrap();

    expect_compiler_error(program);
}

#[test]
fn test_print_parameter_fail_empty() {
    let bytes = include_bytes!("print_parameter_fail_empty.leo");
    let program = parse_program(bytes).unwrap();

    expect_compiler_error(program);
}

#[test]
fn test_print_parameter_fail_none() {
    let bytes = include_bytes!("print_parameter_fail_empty.leo");
    let program = parse_program(bytes).unwrap();

    expect_compiler_error(program);
}

#[test]
fn test_print_input() {
    let bytes = include_bytes!("print_input.leo");
    let mut program = parse_program(bytes).unwrap();

    let main_input = generate_main_input(vec![("a", Some(InputValue::Boolean(true)))]);

    program.set_main_input(main_input);

    assert_satisfied(program);
}

#[test]
fn test_debug() {
    let bytes = include_bytes!("debug.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_error() {
    let bytes = include_bytes!("error.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}
