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


//^
//^ TERM
//^

//> TERM -> FUNCTION
pub fn term<'input>(step: &mut Step<'input>) -> Result<Term<'input>, Error<'input>> {
    let mut numerator = Vec::from([factor(step)?]);
    let mut denominator = Vec::new();
    let mut position = true;
    for (change, factor) in multiple!(step, {
        Ok((optional!(step, choice!(step, b'*', b'/')), factor(step)?))
    }) {        
        if let Some(new) = change {position = new}
        match position {
            false => &mut denominator,
            true => &mut numerator
        }.push(factor);
    }
    return Ok(Term {
        numerator: numerator,
        denominator: denominator
    })
}