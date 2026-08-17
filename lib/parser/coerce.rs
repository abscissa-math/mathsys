//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::step::Step;

//> HEAD -> CRATE
use crate::error::Error;


//^
//^ COERCE
//^

//> COERCE -> FUNCTION
pub fn coerce<'valid, Return>(
    call: fn(&mut Step<'valid>) -> Result<Return, Error<'valid>>
) -> fn(&mut Step<'valid>) -> Result<Return, Error<'valid>> {call}