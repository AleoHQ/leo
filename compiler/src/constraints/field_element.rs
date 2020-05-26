//! Methods to enforce constraints on field elements in a resolved Leo program.

use crate::{errors::FieldElementError, types::InputValue};

use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::curves::TEModelParameters;
use snarkos_models::gadgets::curves::{FieldGadget, FpGadget};
use snarkos_models::gadgets::utilities::alloc::AllocGadget;
use snarkos_models::gadgets::utilities::boolean::Boolean;
use snarkos_models::gadgets::utilities::eq::{ConditionalEqGadget, EqGadget};
use snarkos_models::gadgets::utilities::select::CondSelectGadget;
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};
use std::fmt;

/// A constant or allocated element in the field
#[derive(Clone, PartialEq, Eq)]
pub enum FieldElement<F: Field + PrimeField> {
    Constant(F),
    Allocated(FpGadget<F>),
}

impl<F: Field + PrimeField> FieldElement<F> {
    pub fn new_constant(constant: F) -> Self {
        FieldElement::Constant(constant)
    }

    pub fn new_allocated<P: std::clone::Clone + TEModelParameters, CS: ConstraintSystem<F>>(
        cs: &mut CS,
        name: String,
        private: bool,
        input_value: Option<InputValue<P::BaseField, F>>,
    ) -> Result<Self, FieldElementError> {
        // Check that the parameter value is the correct type
        let field_option = match input_value {
            Some(input) => {
                if let InputValue::Field(fe) = input {
                    Some(fe)
                } else {
                    return Err(FieldElementError::InvalidField(input.to_string()));
                }
            }
            None => None,
        };

        // Check visibility of parameter
        let field_value = if private {
            FpGadget::<F>::alloc(&mut cs.ns(|| name), || {
                field_option.ok_or(SynthesisError::AssignmentMissing)
            })?
        } else {
            FpGadget::<F>::alloc_input(&mut cs.ns(|| name), || {
                field_option.ok_or(SynthesisError::AssignmentMissing)
            })?
        };

        Ok(FieldElement::Allocated(field_value))
    }

    pub fn to_fpgadget<CS: ConstraintSystem<F>>(&self, mut cs: CS) -> FpGadget<F> {
        match self {
            FieldElement::Constant(value) => FpGadget::from(&mut cs, value),
            FieldElement::Allocated(value) => value.clone(),
        }
    }

    pub fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (FieldElement::Constant(fe_1_constant), FieldElement::Constant(fe_2_constant)) => {
                fe_1_constant.eq(&fe_2_constant)
            }
            (FieldElement::Allocated(fe_1_gadget), FieldElement::Allocated(fe_2_gadget)) => {
                fe_1_gadget.eq(&fe_2_gadget)
            }
            (FieldElement::Allocated(fe_gadget), FieldElement::Constant(fe_constant))
            | (FieldElement::Constant(fe_constant), FieldElement::Allocated(fe_gadget)) => {
                match fe_gadget.value {
                    Some(ref value) => value.eq(&fe_constant),
                    None => false,
                }
            }
        }
    }

    pub fn ge(&self, other: &Self) -> bool {
        match (self, other) {
            (FieldElement::Constant(fe_1_constant), FieldElement::Constant(fe_2_constant)) => {
                fe_1_constant.ge(&fe_2_constant)
            }
            (FieldElement::Allocated(fe_1_gadget), FieldElement::Allocated(fe_2_gadget)) => {
                match (fe_1_gadget.value, fe_2_gadget.value) {
                    (Some(ref value_1), Some(ref value_2)) => value_1.ge(&value_2),
                    (_, _) => false,
                }
            }
            (FieldElement::Allocated(fe_gadget), FieldElement::Constant(fe_constant))
            | (FieldElement::Constant(fe_constant), FieldElement::Allocated(fe_gadget)) => {
                match fe_gadget.value {
                    Some(ref value) => value.ge(&fe_constant),
                    None => false,
                }
            }
        }
    }

    pub fn gt(&self, other: &Self) -> bool {
        match (self, other) {
            (FieldElement::Constant(fe_1_constant), FieldElement::Constant(fe_2_constant)) => {
                fe_1_constant.gt(&fe_2_constant)
            }
            (FieldElement::Allocated(fe_1_gadget), FieldElement::Allocated(fe_2_gadget)) => {
                match (fe_1_gadget.value, fe_2_gadget.value) {
                    (Some(value_1), Some(value_2)) => value_1.gt(&value_2),
                    (_, _) => false,
                }
            }
            (FieldElement::Allocated(fe_gadget), FieldElement::Constant(fe_constant))
            | (FieldElement::Constant(fe_constant), FieldElement::Allocated(fe_gadget)) => {
                match fe_gadget.value {
                    Some(ref value) => value.gt(&fe_constant),
                    None => false,
                }
            }
        }
    }

    pub fn le(&self, other: &Self) -> bool {
        match (self, other) {
            (FieldElement::Constant(fe_1_constant), FieldElement::Constant(fe_2_constant)) => {
                fe_1_constant.le(&fe_2_constant)
            }
            (FieldElement::Allocated(fe_1_gadget), FieldElement::Allocated(fe_2_gadget)) => {
                match (fe_1_gadget.value, fe_2_gadget.value) {
                    (Some(ref value_1), Some(ref value_2)) => value_1.le(&value_2),
                    (_, _) => false,
                }
            }
            (FieldElement::Allocated(fe_gadget), FieldElement::Constant(fe_constant))
            | (FieldElement::Constant(fe_constant), FieldElement::Allocated(fe_gadget)) => {
                match fe_gadget.value {
                    Some(ref value) => value.le(&fe_constant),
                    None => false,
                }
            }
        }
    }

    pub fn lt(&self, other: &Self) -> bool {
        match (self, other) {
            (FieldElement::Constant(fe_1_constant), FieldElement::Constant(fe_2_constant)) => {
                fe_1_constant.lt(&fe_2_constant)
            }
            (FieldElement::Allocated(fe_1_gadget), FieldElement::Allocated(fe_2_gadget)) => {
                match (fe_1_gadget.value, fe_2_gadget.value) {
                    (Some(ref value_1), Some(ref value_2)) => value_1.lt(&value_2),
                    (_, _) => false,
                }
            }
            (FieldElement::Allocated(fe_gadget), FieldElement::Constant(fe_constant))
            | (FieldElement::Constant(fe_constant), FieldElement::Allocated(fe_gadget)) => {
                match fe_gadget.value {
                    Some(ref value) => value.lt(&fe_constant),
                    None => false,
                }
            }
        }
    }

    pub fn enforce_add<CS: ConstraintSystem<F>>(
        self,
        cs: &mut CS,
        other: &Self,
    ) -> Result<Self, FieldElementError> {
        Ok(match (self, other) {
            (FieldElement::Constant(fe_1_constant), FieldElement::Constant(fe_2_constant)) => {
                FieldElement::Constant(fe_1_constant.add(&fe_2_constant))
            }
            (FieldElement::Allocated(ref fe_1_gadget), FieldElement::Allocated(fe_2_gadget)) => {
                FieldElement::Allocated(fe_1_gadget.add(cs.ns(|| "field add"), fe_2_gadget)?)
            }
            (FieldElement::Allocated(ref fe_gadget), FieldElement::Constant(ref fe_constant))
            | (FieldElement::Constant(ref fe_constant), FieldElement::Allocated(ref fe_gadget)) => {
                FieldElement::Allocated(fe_gadget.add_constant(cs.ns(|| "field add"), fe_constant)?)
            }
        })
    }

    pub fn enforce_sub<CS: ConstraintSystem<F>>(
        self,
        cs: &mut CS,
        other: &Self,
    ) -> Result<Self, FieldElementError> {
        Ok(match (self, other) {
            (FieldElement::Constant(fe_1_constant), FieldElement::Constant(fe_2_constant)) => {
                FieldElement::Constant(fe_1_constant.sub(&fe_2_constant))
            }
            (FieldElement::Allocated(ref fe_1_gadget), FieldElement::Allocated(fe_2_gadget)) => {
                FieldElement::Allocated(
                    fe_1_gadget.sub(cs.ns(|| "field subtraction"), fe_2_gadget)?,
                )
            }
            (FieldElement::Allocated(ref fe_gadget), FieldElement::Constant(ref fe_constant)) => {
                FieldElement::Allocated(
                    fe_gadget.sub_constant(cs.ns(|| "field subtraction"), fe_constant)?,
                )
            }
            (_, _) => unimplemented!("field subtraction between constant and allocated not impl"),
        })
    }

    pub fn enforce_mul<CS: ConstraintSystem<F>>(
        self,
        cs: &mut CS,
        other: &Self,
    ) -> Result<Self, FieldElementError> {
        Ok(match (self, other) {
            (FieldElement::Constant(fe_1_constant), FieldElement::Constant(fe_2_constant)) => {
                FieldElement::Constant(fe_1_constant.mul(&fe_2_constant))
            }
            (
                FieldElement::Allocated(ref fe_1_gadget),
                FieldElement::Allocated(ref fe_2_gadget),
            ) => FieldElement::Allocated(
                fe_1_gadget.mul(cs.ns(|| "field multiplication"), fe_2_gadget)?,
            ),
            (FieldElement::Allocated(ref fe_gadget), FieldElement::Constant(ref fe_constant))
            | (FieldElement::Constant(ref fe_constant), FieldElement::Allocated(ref fe_gadget)) => {
                FieldElement::Allocated(
                    fe_gadget.mul_by_constant(cs.ns(|| "field multiplication"), fe_constant)?,
                )
            }
        })
    }

    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FieldElement::Constant(ref constant) => write!(f, "{}", constant),
            FieldElement::Allocated(ref allocated) => write!(f, "{:?}", allocated),
        }
    }
}

impl<F: Field + PrimeField> fmt::Display for FieldElement<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

impl<F: Field + PrimeField> fmt::Debug for FieldElement<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

impl<F: Field + PrimeField> EqGadget<F> for FieldElement<F> {}

impl<F: Field + PrimeField> ConditionalEqGadget<F> for FieldElement<F> {
    fn conditional_enforce_equal<CS: ConstraintSystem<F>>(
        &self,
        mut cs: CS,
        other: &Self,
        condition: &Boolean,
    ) -> Result<(), SynthesisError> {
        let value_1 = self.to_fpgadget(&mut cs.ns(|| "first"));
        let value_2 = other.to_fpgadget(&mut cs.ns(|| "second"));
        FpGadget::<F>::conditional_enforce_equal(&value_1, cs, &value_2, condition)
    }

    fn cost() -> usize {
        <FpGadget<F> as ConditionalEqGadget<F>>::cost()
    }
}

impl<F: Field + PrimeField> CondSelectGadget<F> for FieldElement<F> {
    fn conditionally_select<CS: ConstraintSystem<F>>(
        mut cs: CS,
        cond: &Boolean,
        first: &Self,
        second: &Self,
    ) -> Result<Self, SynthesisError> {
        let value_1 = first.to_fpgadget(&mut cs.ns(|| "first"));
        let value_2 = second.to_fpgadget(&mut cs.ns(|| "second"));
        Ok(FieldElement::Allocated(
            FpGadget::<F>::conditionally_select(cs, cond, &value_1, &value_2)?,
        ))
    }

    fn cost() -> usize {
        <FpGadget<F> as CondSelectGadget<F>>::cost()
    }
}
