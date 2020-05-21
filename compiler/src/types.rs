//! A typed Leo program consists of import, circuit, and function definitions.
//! Each defined type consists of typed statements and expressions.

use crate::Import;
use leo_gadgets::integers::{
    uint128::UInt128, uint16::UInt16, uint32::UInt32, uint64::UInt64, uint8::UInt8,
};

use snarkos_models::curves::{Field, Group, PrimeField};
use snarkos_models::gadgets::{r1cs::Variable as R1CSVariable, utilities::boolean::Boolean};
use std::collections::HashMap;
use std::marker::PhantomData;

/// An identifier in the constrained program.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Identifier<F: Field + PrimeField, G: Group> {
    pub name: String,
    pub(crate) _group: PhantomData<G>,
    pub(crate) _engine: PhantomData<F>,
}

impl<F: Field + PrimeField, G: Group> Identifier<F, G> {
    pub fn new(name: String) -> Self {
        Self {
            name,
            _group: PhantomData::<G>,
            _engine: PhantomData::<F>,
        }
    }

    pub fn is_self(&self) -> bool {
        self.name == "Self"
    }
}

/// A variable that is assigned to a value in the constrained program
#[derive(Clone, PartialEq, Eq)]
pub struct Variable<F: Field + PrimeField, G: Group> {
    pub identifier: Identifier<F, G>,
    pub mutable: bool,
    pub _type: Option<Type<F, G>>,
}

/// An integer type enum wrapping the integer value. Used only in expressions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Integer {
    U8(UInt8),
    U16(UInt16),
    U32(UInt32),
    U64(UInt64),
    U128(UInt128),
}

/// A constant or allocated element in the field
#[derive(Clone, PartialEq, Eq)]
pub enum FieldElement<F: Field + PrimeField> {
    Constant(F),
    Allocated(Option<F>, R1CSVariable),
}

/// Range or expression enum
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RangeOrExpression<F: Field + PrimeField, G: Group> {
    Range(Option<Integer>, Option<Integer>),
    Expression(Expression<F, G>),
}

/// Spread or expression
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpreadOrExpression<F: Field + PrimeField, G: Group> {
    Spread(Expression<F, G>),
    Expression(Expression<F, G>),
}

/// Expression that evaluates to a value
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression<F: Field + PrimeField, G: Group> {
    // Identifier
    Identifier(Identifier<F, G>),

    // Values
    Integer(Integer),
    FieldElement(FieldElement<F>),
    GroupElement(G),
    Boolean(Boolean),
    Implicit(String),

    // Number operations
    Add(Box<Expression<F, G>>, Box<Expression<F, G>>),
    Sub(Box<Expression<F, G>>, Box<Expression<F, G>>),
    Mul(Box<Expression<F, G>>, Box<Expression<F, G>>),
    Div(Box<Expression<F, G>>, Box<Expression<F, G>>),
    Pow(Box<Expression<F, G>>, Box<Expression<F, G>>),

    // Boolean operations
    Not(Box<Expression<F, G>>),
    Or(Box<Expression<F, G>>, Box<Expression<F, G>>),
    And(Box<Expression<F, G>>, Box<Expression<F, G>>),
    Eq(Box<Expression<F, G>>, Box<Expression<F, G>>),
    Geq(Box<Expression<F, G>>, Box<Expression<F, G>>),
    Gt(Box<Expression<F, G>>, Box<Expression<F, G>>),
    Leq(Box<Expression<F, G>>, Box<Expression<F, G>>),
    Lt(Box<Expression<F, G>>, Box<Expression<F, G>>),

    // Conditionals
    IfElse(
        Box<Expression<F, G>>,
        Box<Expression<F, G>>,
        Box<Expression<F, G>>,
    ),

    // Arrays
    Array(Vec<Box<SpreadOrExpression<F, G>>>),
    ArrayAccess(Box<Expression<F, G>>, Box<RangeOrExpression<F, G>>), // (array name, range)

    // Circuits
    Circuit(Identifier<F, G>, Vec<CircuitFieldDefinition<F, G>>),
    CircuitMemberAccess(Box<Expression<F, G>>, Identifier<F, G>), // (declared circuit name, circuit member name)
    CircuitStaticFunctionAccess(Box<Expression<F, G>>, Identifier<F, G>), // (defined circuit name, circuit static member name)

    // Functions
    FunctionCall(Box<Expression<F, G>>, Vec<Expression<F, G>>),
}

/// Definition assignee: v, arr[0..2], Point p.x
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Assignee<F: Field + PrimeField, G: Group> {
    Identifier(Identifier<F, G>),
    Array(Box<Assignee<F, G>>, RangeOrExpression<F, G>),
    CircuitField(Box<Assignee<F, G>>, Identifier<F, G>), // (circuit name, circuit field name)
}

/// Explicit integer type
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IntegerType {
    U8,
    U16,
    U32,
    U64,
    U128,
}

/// Explicit type used for defining a variable or expression type
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type<F: Field + PrimeField, G: Group> {
    IntegerType(IntegerType),
    FieldElement,
    GroupElement,
    Boolean,
    Array(Box<Type<F, G>>, Vec<usize>),
    Circuit(Identifier<F, G>),
    SelfType,
}

impl<F: Field + PrimeField, G: Group> Type<F, G> {
    pub fn outer_dimension(&self, dimensions: &Vec<usize>) -> Self {
        let _type = self.clone();

        if dimensions.len() > 1 {
            let mut next = vec![];
            next.extend_from_slice(&dimensions[1..]);

            return Type::Array(Box::new(_type), next);
        }

        _type
    }

    pub fn inner_dimension(&self, dimensions: &Vec<usize>) -> Self {
        let _type = self.clone();

        if dimensions.len() > 1 {
            let mut next = vec![];
            next.extend_from_slice(&dimensions[..dimensions.len() - 1]);

            return Type::Array(Box::new(_type), next);
        }

        _type
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum ConditionalNestedOrEnd<F: Field + PrimeField, G: Group> {
    Nested(Box<ConditionalStatement<F, G>>),
    End(Vec<Statement<F, G>>),
}

#[derive(Clone, PartialEq, Eq)]
pub struct ConditionalStatement<F: Field + PrimeField, G: Group> {
    pub condition: Expression<F, G>,
    pub statements: Vec<Statement<F, G>>,
    pub next: Option<ConditionalNestedOrEnd<F, G>>,
}

/// Program statement that defines some action (or expression) to be carried out.
#[derive(Clone, PartialEq, Eq)]
pub enum Statement<F: Field + PrimeField, G: Group> {
    Return(Vec<Expression<F, G>>),
    Definition(Variable<F, G>, Expression<F, G>),
    Assign(Assignee<F, G>, Expression<F, G>),
    MultipleAssign(Vec<Variable<F, G>>, Expression<F, G>),
    Conditional(ConditionalStatement<F, G>),
    For(Identifier<F, G>, Integer, Integer, Vec<Statement<F, G>>),
    AssertEq(Expression<F, G>, Expression<F, G>),
    Expression(Expression<F, G>),
}

/// Circuits

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CircuitFieldDefinition<F: Field + PrimeField, G: Group> {
    pub identifier: Identifier<F, G>,
    pub expression: Expression<F, G>,
}

#[derive(Clone, PartialEq, Eq)]
pub enum CircuitMember<F: Field + PrimeField, G: Group> {
    CircuitField(Identifier<F, G>, Type<F, G>),
    CircuitFunction(bool, Function<F, G>),
}

#[derive(Clone, PartialEq, Eq)]
pub struct Circuit<F: Field + PrimeField, G: Group> {
    pub identifier: Identifier<F, G>,
    pub members: Vec<CircuitMember<F, G>>,
}

/// Function parameters

#[derive(Clone, PartialEq, Eq)]
pub struct InputModel<F: Field + PrimeField, G: Group> {
    pub identifier: Identifier<F, G>,
    pub mutable: bool,
    pub private: bool,
    pub _type: Type<F, G>,
}

#[derive(Clone, PartialEq, Eq)]
pub enum InputValue<F: Field + PrimeField, G: Group> {
    Integer(usize),
    Field(F),
    Group(G),
    Boolean(bool),
    Array(Vec<InputValue<F, G>>),
}

#[derive(Clone, PartialEq, Eq)]
pub struct Function<F: Field + PrimeField, G: Group> {
    pub function_name: Identifier<F, G>,
    pub inputs: Vec<InputModel<F, G>>,
    pub returns: Vec<Type<F, G>>,
    pub statements: Vec<Statement<F, G>>,
}

impl<F: Field + PrimeField, G: Group> Function<F, G> {
    pub fn get_name(&self) -> String {
        self.function_name.name.clone()
    }
}

/// A simple program with statement expressions, program arguments and program returns.
#[derive(Debug, Clone)]
pub struct Program<F: Field + PrimeField, G: Group> {
    pub name: Identifier<F, G>,
    pub num_parameters: usize,
    pub imports: Vec<Import<F, G>>,
    pub circuits: HashMap<Identifier<F, G>, Circuit<F, G>>,
    pub functions: HashMap<Identifier<F, G>, Function<F, G>>,
}

impl<'ast, F: Field + PrimeField, G: Group> Program<F, G> {
    pub fn new() -> Self {
        Self {
            name: Identifier::new("".into()),
            num_parameters: 0,
            imports: vec![],
            circuits: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.name.clone()
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Identifier::new(name);
        self
    }
}
