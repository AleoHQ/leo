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

use crate::{
    assert_satisfied,
    expect_type_inference_error,
    generate_main_input,
    get_output,
    parse_program,
    parse_program_with_input,
};

use leo_ast::InputValue;
use leo_input::types::{IntegerType, U8Type, UnsignedIntegerType};
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use snarkos_algorithms::prf::blake2s::Blake2s as B2SPRF;
use snarkos_models::algorithms::PRF;

#[test]
fn test_arguments_length_fail() {
    let program_bytes = include_bytes!("arguments_length_fail.leo");
    let error = parse_program(program_bytes).err().unwrap();

    expect_type_inference_error(error);
}

#[test]
fn test_arguments_type_fail() {
    let program_bytes = include_bytes!("arguments_type_fail.leo");
    let error = parse_program(program_bytes).err().unwrap();

    expect_type_inference_error(error);
}

#[test]
fn test_blake2s_input() {
    let input_bytes = include_bytes!("inputs/valid_input.in");
    let program_bytes = include_bytes!("blake2s_input.leo");
    let expected_bytes = include_bytes!("outputs/valid_output.out");

    let program = parse_program_with_input(program_bytes, input_bytes).unwrap();

    let expected = std::str::from_utf8(expected_bytes).unwrap();
    let actual_bytes = get_output(program);
    let actual = std::str::from_utf8(actual_bytes.bytes().as_slice()).unwrap();

    assert_eq!(expected, actual)
}

#[test]
fn test_blake2s_random() {
    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

    let mut seed = [0u8; 32];
    rng.fill(&mut seed);

    let mut message = [0u8; 32];
    rng.fill(&mut message);

    // Use snarkos-algorithms blake2s evaluate to get expected value
    let expected = B2SPRF::evaluate(&seed, &message).unwrap().to_vec();

    // Create program input values for seed, message, and expected values
    let seed_input_value = bytes_gadget_to_input(seed.to_vec());
    let message_input_value = bytes_gadget_to_input(message.to_vec());
    let expected_value = bytes_gadget_to_input(expected);

    // The `blake2s_random.leo` program will compute a blake2s hash digest and compare it against
    // the expected value
    let bytes = include_bytes!("blake2s_random.leo");
    let mut program = parse_program(bytes).unwrap();

    let main_input = generate_main_input(vec![
        ("seed", Some(seed_input_value)),
        ("message", Some(message_input_value)),
        ("expected", Some(expected_value)),
    ]);

    // Load input values into Leo program
    program.set_main_input(main_input);

    assert_satisfied(program);
}

fn bytes_gadget_to_input(bytes: Vec<u8>) -> InputValue {
    let u8_type = IntegerType::Unsigned(UnsignedIntegerType::U8Type(U8Type {}));
    let bytes = bytes
        .into_iter()
        .map(|byte| InputValue::Integer(u8_type.clone(), byte.to_string()))
        .collect::<Vec<_>>();

    InputValue::Array(bytes)
}
