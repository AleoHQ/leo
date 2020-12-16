use crate::Span;
use crate::{ Expression, Node, Type, ExpressionNode, FromAst, Scope, AsgConvertError, ConstValue, ConstInt };
use std::sync::{ Weak, Arc };
use std::cell::RefCell;
use leo_ast::IntegerType;

pub struct ArrayRangeAccessExpression {
    pub parent: RefCell<Option<Weak<Expression>>>,
    pub span: Option<Span>,
    pub array: Arc<Expression>,
    pub left: Option<Arc<Expression>>,
    pub right: Option<Arc<Expression>>,
}

impl Node for ArrayRangeAccessExpression {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl ExpressionNode for ArrayRangeAccessExpression {
    fn set_parent(&self, parent: Weak<Expression>) {
        self.parent.replace(Some(parent));
    }

    fn get_parent(&self) -> Option<Arc<Expression>> {
        self.parent.borrow().as_ref().map(Weak::upgrade).flatten()
    }

    fn enforce_parents(&self, expr: &Arc<Expression>) {
        self.array.set_parent(Arc::downgrade(expr));
        self.array.enforce_parents(&self.array);
        if let Some(left) = self.left.as_ref() {
            left.set_parent(Arc::downgrade(expr));
        }
        if let Some(right) = self.right.as_ref() {
            right.set_parent(Arc::downgrade(expr));
        }
    }

    fn get_type(&self) -> Option<Type> {
        let (element, array_len) = match self.array.get_type() {
            Some(Type::Array(element, len)) => (element, len),
            _ => return None,
        };
        let const_left = match self.left.as_ref().map(|x| x.const_value()) {
            Some(Some(ConstValue::Int(ConstInt::U32(x)))) => x,
            None => 0,
            _ => return None,
        };
        let const_right = match self.right.as_ref().map(|x| x.const_value()) {
            Some(Some(ConstValue::Int(ConstInt::U32(x)))) => x,
            None => array_len as u32,
            _ => return None,
        };
        if const_left > const_right || const_right as usize > array_len {
            return None;
        }

        Some(Type::Array(element, (const_right - const_left) as usize))
    }

    fn const_value(&self) -> Option<ConstValue> {
        let mut array = match self.array.const_value()? {
            ConstValue::Array(values) => values,
            _ => return None,
        };
        let const_left = match self.left.as_ref().map(|x| x.const_value()) {
            Some(Some(ConstValue::Int(ConstInt::U32(x)))) => x,
            None => 0,
            _ => return None,
        };
        let const_right = match self.right.as_ref().map(|x| x.const_value()) {
            Some(Some(ConstValue::Int(ConstInt::U32(x)))) => x,
            None => array.len() as u32,
            _ => return None,
        };
        if const_left > const_right || const_right as usize > array.len() {
            return None;
        }

        Some(ConstValue::Array(array.drain(const_left as usize..const_right as usize).collect()))
    }
}

impl FromAst<leo_ast::ArrayRangeAccessExpression> for ArrayRangeAccessExpression {
    fn from_ast(scope: &Scope, value: &leo_ast::ArrayRangeAccessExpression, expected_type: Option<Type>) -> Result<ArrayRangeAccessExpression, AsgConvertError> {
        let array = Arc::<Expression>::from_ast(scope, &*value.array, None)?; // todo: partial type expectations here
        let array_type = array.get_type();
        match (expected_type, array_type) {
            (Some(Type::Array(expected_item_type, len)), Some(Type::Array(item_type, current_len))) => {
                if !expected_item_type.is_assignable_from(&*item_type) {
                    return Err(AsgConvertError::unexpected_type(&expected_item_type.to_string(), Some(&*item_type.to_string()), &value.span));
                }
                //todo: how to resolve slicing dimensions at compile time??
            },
            (Some(expected), type_) =>
                return Err(AsgConvertError::unexpected_type(&expected.to_string(), type_.map(|x| x.to_string()).as_deref(), &value.span)),
            (None, Some(Type::Array(_, _))) => (),
            (None, type_) =>
                return Err(AsgConvertError::unexpected_type("array", type_.map(|x| x.to_string()).as_deref(), &value.span)),
        }
        Ok(ArrayRangeAccessExpression {
            parent: RefCell::new(None),
            span: Some(value.span.clone()),
            array,
            left: value.left.as_deref().map(|left| Arc::<Expression>::from_ast(scope, left, Some(Type::Integer(IntegerType::U32)))).transpose()?,
            right: value.right.as_deref().map(|right| Arc::<Expression>::from_ast(scope, right, Some(Type::Integer(IntegerType::U32)))).transpose()?,
        })
    }
}

impl Into<leo_ast::ArrayRangeAccessExpression> for &ArrayRangeAccessExpression {
    fn into(self) -> leo_ast::ArrayRangeAccessExpression {
        leo_ast::ArrayRangeAccessExpression {
            array: Box::new(self.array.as_ref().into()),
            left: self.left.as_ref().map(|left| Box::new(left.as_ref().into())),
            right: self.right.as_ref().map(|right| Box::new(right.as_ref().into())),
            span: self.span.clone().unwrap_or_default(),
        }
    }
}