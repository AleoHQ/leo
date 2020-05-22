//! Module containing methods to enforce constraints in an Leo program

pub mod boolean;
pub use boolean::*;

pub mod function;
pub use function::*;

pub mod expression;
pub use expression::*;

pub mod import;
pub use import::*;

pub mod integer;
pub use integer::*;

pub mod field_element;
pub use field_element::*;

pub mod group_element;
pub use group_element::*;

pub mod program;
pub use program::*;

pub mod value;
pub use value::*;

pub mod statement;
pub use statement::*;

use crate::{
    errors::CompilerError,
    types::{InputValue, Program},
};

use snarkos_models::curves::TEModelParameters;
use snarkos_models::gadgets::curves::FieldGadget;
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

pub fn generate_constraints<
    P: std::clone::Clone + TEModelParameters,
    F: Field + PrimeField + std::borrow::Borrow<P::BaseField>,
    FG: FieldGadget<P::BaseField, F>,
    CS: ConstraintSystem<F>,
>(
    cs: &mut CS,
    program: Program<P::BaseField, F>,
    parameters: Vec<Option<InputValue<P::BaseField, F>>>,
) -> Result<ConstrainedValue<P, F, FG>, CompilerError> {
    let mut resolved_program = ConstrainedProgram::new();
    let program_name = program.get_name();
    let main_function_name = new_scope(program_name.clone(), "main".into());

    resolved_program.resolve_definitions(cs, program)?;

    let main = resolved_program
        .get(&main_function_name)
        .ok_or_else(|| CompilerError::NoMain)?;

    match main.clone() {
        ConstrainedValue::Function(_circuit_identifier, function) => {
            let result =
                resolved_program.enforce_main_function(cs, program_name, function, parameters)?;
            log::debug!("{}", result);
            Ok(result)
        }
        _ => Err(CompilerError::NoMainFunction),
    }
}
