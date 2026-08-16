//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::state::State;

//> HEAD -> CRATE
use crate::error::Error;


//^
//^ CHECK
//^

//> CHECK -> FUNCTION
pub fn check<'valid, Return>(
    call: fn(&mut State<'valid>) -> Result<Return, Error<'valid>>
) -> fn(&mut State<'valid>) -> Result<Return, Error<'valid>> {call}