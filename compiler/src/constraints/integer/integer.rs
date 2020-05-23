//! Methods to enforce constraints on integers in a resolved Leo program.

use crate::{
    constraints::{ConstrainedProgram, ConstrainedValue},
    errors::IntegerError,
    types::{InputValue, Integer},
    IntegerType,
};
use leo_gadgets::integers::{
    uint128::UInt128, uint16::UInt16, uint32::UInt32, uint64::UInt64, uint8::UInt8,
};

use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::curves::TEModelParameters;
use snarkos_models::gadgets::curves::FieldGadget;
use snarkos_models::gadgets::utilities::eq::{ConditionalEqGadget, EqGadget};
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::{boolean::Boolean, select::CondSelectGadget},
    },
};

impl Integer {
    pub fn to_usize(&self) -> usize {
        match self {
            Integer::U8(u8) => u8.value.unwrap() as usize,
            Integer::U16(u16) => u16.value.unwrap() as usize,
            Integer::U32(u32) => u32.value.unwrap() as usize,
            Integer::U64(u64) => u64.value.unwrap() as usize,
            Integer::U128(u128) => u128.value.unwrap() as usize,
        }
    }

    pub(crate) fn get_type(&self) -> IntegerType {
        match self {
            Integer::U8(_u8) => IntegerType::U8,
            Integer::U16(_u16) => IntegerType::U16,
            Integer::U32(_u32) => IntegerType::U32,
            Integer::U64(_u64) => IntegerType::U64,
            Integer::U128(_u128) => IntegerType::U128,
        }
    }
}

impl<F: Field + PrimeField> EqGadget<F> for Integer {}

impl<F: Field + PrimeField> ConditionalEqGadget<F> for Integer {
    fn conditional_enforce_equal<CS: ConstraintSystem<F>>(
        &self,
        mut cs: CS,
        other: &Self,
        condition: &Boolean,
    ) -> Result<(), SynthesisError> {
        match (self, other) {
            (Integer::U8(left_u8), Integer::U8(right_u8)) => {
                left_u8.conditional_enforce_equal(&mut cs.ns(|| "u8 eq"), &right_u8, condition)
            }
            (Integer::U16(left_u16), Integer::U16(right_u16)) => {
                left_u16.conditional_enforce_equal(&mut cs.ns(|| "u16 eq"), &right_u16, condition)
            }
            (Integer::U32(left_u32), Integer::U32(right_u32)) => {
                left_u32.conditional_enforce_equal(&mut cs.ns(|| "u32 eq"), &right_u32, condition)
            }
            (Integer::U64(left_u64), Integer::U64(right_u64)) => {
                left_u64.conditional_enforce_equal(&mut cs.ns(|| "u64 eq"), &right_u64, condition)
            }
            (Integer::U128(left_u128), Integer::U128(right_u128)) => left_u128
                .conditional_enforce_equal(&mut cs.ns(|| "u128 eq"), &right_u128, condition),
            (_, _) => Err(SynthesisError::Unsatisfiable),
        }
    }

    fn cost() -> usize {
        <UInt128 as ConditionalEqGadget<F>>::cost()
    }
}

impl<F: Field + PrimeField> CondSelectGadget<F> for Integer {
    fn conditionally_select<CS: ConstraintSystem<F>>(
        cs: CS,
        cond: &Boolean,
        first: &Self,
        second: &Self,
    ) -> Result<Self, SynthesisError> {
        match (first, second) {
            (Integer::U8(u8_first), Integer::U8(u8_second)) => Ok(Integer::U8(
                UInt8::conditionally_select(cs, cond, u8_first, u8_second)?,
            )),
            (Integer::U16(u16_first), Integer::U16(u18_second)) => Ok(Integer::U16(
                UInt16::conditionally_select(cs, cond, u16_first, u18_second)?,
            )),
            (Integer::U32(u32_first), Integer::U32(u32_second)) => Ok(Integer::U32(
                UInt32::conditionally_select(cs, cond, u32_first, u32_second)?,
            )),
            (Integer::U64(u64_first), Integer::U64(u64_second)) => Ok(Integer::U64(
                UInt64::conditionally_select(cs, cond, u64_first, u64_second)?,
            )),
            (Integer::U128(u128_first), Integer::U128(u128_second)) => Ok(Integer::U128(
                UInt128::conditionally_select(cs, cond, u128_first, u128_second)?,
            )),
            (_, _) => Err(SynthesisError::Unsatisfiable), // types do not match
        }
    }

    fn cost() -> usize {
        <UInt128 as CondSelectGadget<F>>::cost()
    }
}

impl<
        P: std::clone::Clone + TEModelParameters,
        F: Field + PrimeField,
        FG: FieldGadget<P::BaseField, F>,
        CS: ConstraintSystem<F>,
    > ConstrainedProgram<P, F, FG, CS>
{
    pub(crate) fn integer_from_parameter(
        &mut self,
        cs: &mut CS,
        integer_type: IntegerType,
        name: String,
        private: bool,
        integer_value: Option<InputValue<P::BaseField, F>>,
    ) -> Result<ConstrainedValue<P, F, FG>, IntegerError> {
        // Check that the input value is the correct type
        let integer_option = match integer_value {
            Some(input) => {
                if let InputValue::Integer(integer) = input {
                    Some(integer)
                } else {
                    return Err(IntegerError::InvalidInteger(input.to_string()));
                }
            }
            None => None,
        };

        match integer_type {
            IntegerType::U8 => self.u8_from_input(cs, name, private, integer_option),
            IntegerType::U16 => self.u16_from_input(cs, name, private, integer_option),
            IntegerType::U32 => self.u32_from_input(cs, name, private, integer_option),
            IntegerType::U64 => self.u64_from_input(cs, name, private, integer_option),
            IntegerType::U128 => self.u128_from_input(cs, name, private, integer_option),
        }
    }

    pub(crate) fn enforce_integer_add(
        cs: &mut CS,
        left: Integer,
        right: Integer,
    ) -> Result<ConstrainedValue<P, F, FG>, IntegerError> {
        Ok(ConstrainedValue::Integer(match (left, right) {
            (Integer::U8(left_u8), Integer::U8(right_u8)) => {
                Integer::U8(Self::enforce_u8_add(cs, left_u8, right_u8)?)
            }
            (Integer::U16(left_u16), Integer::U16(right_u16)) => {
                Integer::U16(Self::enforce_u16_add(cs, left_u16, right_u16)?)
            }
            (Integer::U32(left_u32), Integer::U32(right_u32)) => {
                Integer::U32(Self::enforce_u32_add(cs, left_u32, right_u32)?)
            }
            (Integer::U64(left_u64), Integer::U64(right_u64)) => {
                Integer::U64(Self::enforce_u64_add(cs, left_u64, right_u64)?)
            }
            (Integer::U128(left_u128), Integer::U128(right_u128)) => {
                Integer::U128(Self::enforce_u128_add(cs, left_u128, right_u128)?)
            }
            (left, right) => {
                return Err(IntegerError::CannotEnforce(format!("{} + {}", left, right)))
            }
        }))
    }
    pub(crate) fn enforce_integer_sub(
        cs: &mut CS,
        left: Integer,
        right: Integer,
    ) -> Result<ConstrainedValue<P, F, FG>, IntegerError> {
        Ok(ConstrainedValue::Integer(match (left, right) {
            (Integer::U8(left_u8), Integer::U8(right_u8)) => {
                Integer::U8(Self::enforce_u8_sub(cs, left_u8, right_u8)?)
            }
            (Integer::U16(left_u16), Integer::U16(right_u16)) => {
                Integer::U16(Self::enforce_u16_sub(cs, left_u16, right_u16)?)
            }
            (Integer::U32(left_u32), Integer::U32(right_u32)) => {
                Integer::U32(Self::enforce_u32_sub(cs, left_u32, right_u32)?)
            }
            (Integer::U64(left_u64), Integer::U64(right_u64)) => {
                Integer::U64(Self::enforce_u64_sub(cs, left_u64, right_u64)?)
            }
            (Integer::U128(left_u128), Integer::U128(right_u128)) => {
                Integer::U128(Self::enforce_u128_sub(cs, left_u128, right_u128)?)
            }
            (left, right) => {
                return Err(IntegerError::CannotEnforce(format!("{} - {}", left, right)))
            }
        }))
    }
    pub(crate) fn enforce_integer_mul(
        cs: &mut CS,
        left: Integer,
        right: Integer,
    ) -> Result<ConstrainedValue<P, F, FG>, IntegerError> {
        Ok(ConstrainedValue::Integer(match (left, right) {
            (Integer::U8(left_u8), Integer::U8(right_u8)) => {
                Integer::U8(Self::enforce_u8_mul(cs, left_u8, right_u8)?)
            }
            (Integer::U16(left_u16), Integer::U16(right_u16)) => {
                Integer::U16(Self::enforce_u16_mul(cs, left_u16, right_u16)?)
            }
            (Integer::U32(left_u32), Integer::U32(right_u32)) => {
                Integer::U32(Self::enforce_u32_mul(cs, left_u32, right_u32)?)
            }
            (Integer::U64(left_u64), Integer::U64(right_u64)) => {
                Integer::U64(Self::enforce_u64_mul(cs, left_u64, right_u64)?)
            }
            (Integer::U128(left_u128), Integer::U128(right_u128)) => {
                Integer::U128(Self::enforce_u128_mul(cs, left_u128, right_u128)?)
            }
            (left, right) => {
                return Err(IntegerError::CannotEnforce(format!("{} * {}", left, right)))
            }
        }))
    }
    pub(crate) fn enforce_integer_div(
        cs: &mut CS,
        left: Integer,
        right: Integer,
    ) -> Result<ConstrainedValue<P, F, FG>, IntegerError> {
        Ok(ConstrainedValue::Integer(match (left, right) {
            (Integer::U8(left_u8), Integer::U8(right_u8)) => {
                Integer::U8(Self::enforce_u8_div(cs, left_u8, right_u8)?)
            }
            (Integer::U16(left_u16), Integer::U16(right_u16)) => {
                Integer::U16(Self::enforce_u16_div(cs, left_u16, right_u16)?)
            }
            (Integer::U32(left_u32), Integer::U32(right_u32)) => {
                Integer::U32(Self::enforce_u32_div(cs, left_u32, right_u32)?)
            }
            (Integer::U64(left_u64), Integer::U64(right_u64)) => {
                Integer::U64(Self::enforce_u64_div(cs, left_u64, right_u64)?)
            }
            (Integer::U128(left_u128), Integer::U128(right_u128)) => {
                Integer::U128(Self::enforce_u128_div(cs, left_u128, right_u128)?)
            }
            (left, right) => {
                return Err(IntegerError::CannotEnforce(format!("{} / {}", left, right)))
            }
        }))
    }
    pub(crate) fn enforce_integer_pow(
        cs: &mut CS,
        left: Integer,
        right: Integer,
    ) -> Result<ConstrainedValue<P, F, FG>, IntegerError> {
        Ok(ConstrainedValue::Integer(match (left, right) {
            (Integer::U8(left_u8), Integer::U8(right_u8)) => {
                Integer::U8(Self::enforce_u8_pow(cs, left_u8, right_u8)?)
            }
            (Integer::U16(left_u16), Integer::U16(right_u16)) => {
                Integer::U16(Self::enforce_u16_pow(cs, left_u16, right_u16)?)
            }
            (Integer::U32(left_u32), Integer::U32(right_u32)) => {
                Integer::U32(Self::enforce_u32_pow(cs, left_u32, right_u32)?)
            }
            (Integer::U64(left_u64), Integer::U64(right_u64)) => {
                Integer::U64(Self::enforce_u64_pow(cs, left_u64, right_u64)?)
            }
            (Integer::U128(left_u128), Integer::U128(right_u128)) => {
                Integer::U128(Self::enforce_u128_pow(cs, left_u128, right_u128)?)
            }
            (left, right) => {
                return Err(IntegerError::CannotEnforce(format!(
                    "{} ** {}",
                    left, right
                )))
            }
        }))
    }
}
