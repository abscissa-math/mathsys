//^
//^ HEAD
//^

//> HEAD -> CRATE
use crate::error::Error;


//^
//^ RUNTIME
//^

//> RUNTIME -> TRAIT
pub const trait Runtime<'valid> {
    fn resolve(&'valid self, module: &'valid str) -> &'valid [u8];
    fn warning(error: Error<'valid>) -> ();
    fn failure(error: Error<'valid>) -> !;
}