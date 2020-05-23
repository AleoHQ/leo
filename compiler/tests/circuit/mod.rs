use crate::{compile_program, get_error, get_output, integer::u32::output_one};

use leo_compiler::{
    compiler::Compiler,
    errors::{CompilerError, ExpressionError, FunctionError, StatementError},
    ConstrainedCircuitMember,
    ConstrainedValue,
    Expression,
    Function,
    Identifier,
    Integer,
    Statement,
    Type,
};
use snarkos_curves::{bls12_377::Fr, edwards_bls12::EdwardsProjective};
use snarkos_models::gadgets::utilities::uint32::UInt32;

const DIRECTORY_NAME: &str = "tests/circuit/";

// Circ { x: 1u32 }
fn output_circuit(program: Compiler<Fr, EdwardsProjective>) {
    let output = get_output(program);
    assert_eq!(
        ConstrainedValue::<Fr, EdwardsProjective>::Return(vec![ConstrainedValue::CircuitExpression(
            Identifier::new("Circ".into()),
            vec![ConstrainedCircuitMember(
                Identifier::new("x".into()),
                ConstrainedValue::Integer(Integer::U32(UInt32::constant(1u32)))
            )]
        )]),
        output
    );
}

fn fail_expected_member(program: Compiler<Fr, EdwardsProjective>) {
    match get_error(program) {
        CompilerError::FunctionError(FunctionError::StatementError(StatementError::ExpressionError(
            ExpressionError::ExpectedCircuitMember(_string),
        ))) => {}
        error => panic!("Expected invalid circuit member error, got {}", error),
    }
}

fn fail_undefined_member(program: Compiler<Fr, EdwardsProjective>) {
    match get_error(program) {
        CompilerError::FunctionError(FunctionError::StatementError(StatementError::ExpressionError(
            ExpressionError::UndefinedMemberAccess(_, _),
        ))) => {}
        error => panic!("Expected undefined circuit member error, got {}", error),
    }
}

// Expressions

#[test]
fn test_inline() {
    let program = compile_program(DIRECTORY_NAME, "inline.leo").unwrap();
    output_circuit(program);
}

#[test]
fn test_inline_fail() {
    let program = compile_program(DIRECTORY_NAME, "inline_fail.leo").unwrap();
    fail_expected_member(program)
}

#[test]
fn test_inline_undefined() {
    let program = compile_program(DIRECTORY_NAME, "inline_undefined.leo").unwrap();
    match get_error(program) {
        CompilerError::FunctionError(FunctionError::StatementError(StatementError::ExpressionError(
            ExpressionError::UndefinedCircuit(_),
        ))) => {}
        error => panic!("Expected undefined circuit error, got {}", error),
    }
}

// Members

#[test]
fn test_member_field() {
    let program = compile_program(DIRECTORY_NAME, "member_field.leo").unwrap();
    output_one(program);
}

#[test]
fn test_member_field_fail() {
    let program = compile_program(DIRECTORY_NAME, "member_field_fail.leo").unwrap();
    fail_undefined_member(program);
}

#[test]
fn test_member_function() {
    let program = compile_program(DIRECTORY_NAME, "member_function.leo").unwrap();
    output_one(program);
}

#[test]
fn test_member_function_fail() {
    let program = compile_program(DIRECTORY_NAME, "member_function_fail.leo").unwrap();
    fail_undefined_member(program);
}

#[test]
fn test_member_function_invalid() {
    let program = compile_program(DIRECTORY_NAME, "member_function_invalid.leo").unwrap();
    match get_error(program) {
        CompilerError::FunctionError(FunctionError::StatementError(StatementError::ExpressionError(
            ExpressionError::InvalidStaticAccess(_),
        ))) => {}
        error => panic!("Expected invalid function error, got {}", error),
    }
}

#[test]
fn test_member_static_function() {
    let program = compile_program(DIRECTORY_NAME, "member_static_function.leo").unwrap();
    output_one(program);
}

#[test]
fn test_member_static_function_undefined() {
    let program = compile_program(DIRECTORY_NAME, "member_static_function_undefined.leo").unwrap();
    match get_error(program) {
        CompilerError::FunctionError(FunctionError::StatementError(StatementError::ExpressionError(
            ExpressionError::UndefinedStaticAccess(_, _),
        ))) => {}
        error => panic!("Expected undefined static function error, got {}", error),
    }
}
#[test]
fn test_member_static_function_invalid() {
    let program = compile_program(DIRECTORY_NAME, "member_static_function_invalid.leo").unwrap();
    match get_error(program) {
        CompilerError::FunctionError(FunctionError::StatementError(StatementError::ExpressionError(
            ExpressionError::InvalidMemberAccess(_),
        ))) => {}
        error => panic!("Expected invalid static function error, got {}", error),
    }
}

// Self
#[test]
fn test_self() {
    let program = compile_program(DIRECTORY_NAME, "self.leo").unwrap();
    let output = get_output(program);

    // circuit Circ {
    //   static function new() -> Self {
    //     return Self { }
    //   }
    // }
    assert_eq!(
        ConstrainedValue::<Fr, EdwardsProjective>::Return(vec![ConstrainedValue::CircuitExpression(
            Identifier::new("Circ".into()),
            vec![ConstrainedCircuitMember(
                Identifier::new("new".into()),
                ConstrainedValue::Static(Box::new(ConstrainedValue::Function(
                    Some(Identifier::new("Circ".into())),
                    Function {
                        function_name: Identifier::new("new".into()),
                        inputs: vec![],
                        returns: vec![Type::SelfType],
                        statements: vec![Statement::Return(vec![Expression::Circuit(
                            Identifier::new("Self".into()),
                            vec![]
                        )])]
                    }
                )))
            )]
        )]),
        output
    );
}

// All

// #[test]
// fn test_pedersen_mock() {
//     let program = compile_program(DIRECTORY_NAME, "pedersen_mock.leo").unwrap();
//     output_zero(program);
// }
