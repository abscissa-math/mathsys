//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    step::Step,
    value::value,
    consumers::keyword,
    quantifiers::optional
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
    step: &mut Step<'input>
) -> Result<Factor<'input>, Error<'input>> {return Ok(Factor {
    value: value(step)?,
    exponent: optional!({
        keyword!(step, [b'^'])?;
        let expression = expression(step)?;
        keyword!(step, [b'^'])?;
        Ok(expression)
    }, step)
})}