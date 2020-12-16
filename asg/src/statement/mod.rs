mod return_;
pub use return_::*;
mod definition;
pub use definition::*;
mod assign;
pub use assign::*;
mod conditional;
pub use conditional::*;
mod iteration;
pub use iteration::*;
mod console;
pub use console::*;
mod expression;
pub use expression::*;
mod block;
pub use block::*;

use crate::{ AsgConvertError, Scope, FromAst, Type };

pub enum Statement {
    Return(ReturnStatement),
    Definition(DefinitionStatement),
    Assign(AssignStatement),
    Conditional(ConditionalStatement),
    Iteration(IterationStatement),
    Console(ConsoleStatement),
    Expression(ExpressionStatement),
    Block(BlockStatement),
}

impl FromAst<leo_ast::Statement> for Statement {
    fn from_ast(scope: &Scope, value: &leo_ast::Statement, _expected_type: Option<Type>) -> Result<Self, AsgConvertError> {
        use leo_ast::Statement::*;
        Ok(match value {
            Return(statement) => {
                Statement::Return(ReturnStatement::from_ast(scope, statement, None)?)
            },
            Definition(statement) => {
                Statement::Definition(DefinitionStatement::from_ast(scope, statement, None)?)
            },
            Assign(statement) => {
                Statement::Assign(AssignStatement::from_ast(scope, statement, None)?)
            },
            Conditional(statement) => {
                Statement::Conditional(ConditionalStatement::from_ast(scope, statement, None)?)
            },
            Iteration(statement) => {
                Statement::Iteration(IterationStatement::from_ast(scope, statement, None)?)
            },
            Console(statement) => {
                Statement::Console(ConsoleStatement::from_ast(scope, statement, None)?)
            },
            Expression(statement) => {
                Statement::Expression(ExpressionStatement::from_ast(scope, statement, None)?)
            },
            Block(statement) => {
                Statement::Block(BlockStatement::from_ast(scope, statement, None)?)
            },
        })
    }
}

impl Into<leo_ast::Statement> for &Statement {
    fn into(self) -> leo_ast::Statement {
        use Statement::*;
        match self {
            Return(statement) => leo_ast::Statement::Return(statement.into()),
            Definition(statement) => leo_ast::Statement::Definition(statement.into()),
            Assign(statement) => leo_ast::Statement::Assign(statement.into()),
            Conditional(statement) => leo_ast::Statement::Conditional(statement.into()),
            Iteration(statement) => leo_ast::Statement::Iteration(statement.into()),
            Console(statement) => leo_ast::Statement::Console(statement.into()),
            Expression(statement) => leo_ast::Statement::Expression(statement.into()),
            Block(statement) => leo_ast::Statement::Block(statement.into()),
        }
    }
}