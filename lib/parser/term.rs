//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    state::State,
    factor::factor,
    choice::choice,
    optional::optional,
    multiple::multiple
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
pub fn term<'input>(state: &mut State<'input>) -> Result<Term<'input>, Error<'input>> {
    let mut numerator = Vec::from([factor(state)?]);
    let mut denominator = Vec::new();
    let mut position = true;
    for (change, factor) in multiple!(state, {
        Ok((optional!(state, choice!(state, b'*', b'/')), factor(state)?))
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