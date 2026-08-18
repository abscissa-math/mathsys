//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    step::Step,
    factor::factor,
    consumers::choice,
    quantifiers::{
        optional,
        multiple
    }
};

//> HEAD -> CRATE
use crate::{
    error::Error,
    syntax::term::Term
};

//> HEAD -> NONEMPTY
use nonempty::NonEmpty;


//^
//^ TERM
//^

//> TERM -> FUNCTION
pub fn term<'input>(step: &mut Step<'input>) -> Result<Term<'input>, Error<'input>> {
    let first = factor(step)?;
    let mut numerator = Vec::new();
    let mut denominator = Vec::new();
    let mut position = true;
    for (change, factor) in multiple!({
        Ok((optional!(choice!(step, b'*', b'/'), step), factor(step)?))
    }, step) {        
        if let Some(new) = change {position = new}
        match position {
            false => &mut denominator,
            true => &mut numerator
        }.push(factor);
    }
    return Ok(Term {
        numerator: Box::new(NonEmpty::from((first, numerator))),
        denominator: denominator
    })
}