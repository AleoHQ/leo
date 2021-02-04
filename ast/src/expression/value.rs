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

use super::*;
use crate::GroupTuple;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValueExpression {
    // todo: deserialize values here
    Address(String, Span),
    Boolean(String, Span),
    Field(String, Span),
    Group(Box<GroupValue>),
    Implicit(String, Span),
    Integer(IntegerType, String, Span),
}

impl fmt::Display for ValueExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ValueExpression::*;
        match &self {
            Address(address, _) => write!(f, "{}", address),
            Boolean(boolean, _) => write!(f, "{}", boolean),
            Field(field, _) => write!(f, "{}", field),
            Implicit(implicit, _) => write!(f, "{}", implicit),
            Integer(value, type_, _) => write!(f, "{}{}", value, type_),
            Group(group) => write!(f, "{}", group),
        }
    }
}

impl Node for ValueExpression {
    fn span(&self) -> &Span {
        use ValueExpression::*;
        match &self {
            Address(_, span) | Boolean(_, span) | Field(_, span) | Implicit(_, span) | Integer(_, _, span) => span,
            Group(group) => match &**group {
                GroupValue::Single(_, span) | GroupValue::Tuple(GroupTuple { span, .. }) => span,
            },
        }
    }

    fn set_span(&mut self, new_span: Span) {
        use ValueExpression::*;
        match self {
            Address(_, span) | Boolean(_, span) | Field(_, span) | Implicit(_, span) | Integer(_, _, span) => {
                *span = new_span
            }
            Group(group) => match &mut **group {
                GroupValue::Single(_, span) | GroupValue::Tuple(GroupTuple { span, .. }) => *span = new_span,
            },
        }
    }
}
