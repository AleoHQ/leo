use crate::{ast::Rule, expressions::Expression, values::PositiveNumber};

use pest::Span;
use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::expression_array_initializer))]
pub struct ArrayInitializerExpression<'ast> {
    pub expression: Box<Expression<'ast>>,
    pub count: PositiveNumber<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}
