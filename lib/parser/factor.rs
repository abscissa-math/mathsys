//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    state::State,
    value::value,
    keyword::keyword,
    optional::optional
};

//> HEAD -> CRATE
use crate::{
    error::Error, 
    parser::expression::expression, 
    syntax::factor::Factor
};


//^
//^ FACTOR
//^

//> FACTOR -> FUNCTION
pub fn factor<'input>(
    state: &mut State<'input>
) -> Result<Factor<'input>, Error<'input>> {return Ok(Factor {
    value: value(state)?,
    exponent: optional!(state, {
        keyword!(state, [b'^'])?;
        let expression = expression(state)?;
        keyword!(state, [b'^'])?;
        Ok(expression)
    })
})}