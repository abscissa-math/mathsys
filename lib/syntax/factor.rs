//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    expression::Expression,
    value::Value
};


//^
//^ FACTOR
//^

//> FACTOR -> STRUCT
pub struct Factor<'valid> {
    pub value: Value<'valid>,
    pub exponent: Option<Expression<'valid>>
}