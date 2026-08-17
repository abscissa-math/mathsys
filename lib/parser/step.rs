//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::state::State;

//> HEAD -> CRATE
use crate::error::Error;


//^
//^ STEP
//^

//> STEP -> STRUCT
pub struct Step<'valid> {
    pub input: &'valid [u8],
    pub state: State<'valid>
}

//> STEP -> IMPLEMENTATION
impl<'valid> Step<'valid> {
    pub fn depleted(&self, error: Error<'valid>) -> Result<(), Error<'valid>> {
        return if self.input.len() == self.state.position.index {Ok(())} else {Err(error)};
    }
}

//> STEP -> FROM TOKENS
impl<'valid> From<&'valid [u8]> for Step<'valid> {
    fn from(value: &'valid [u8]) -> Self {return Self {
        input: value,
        state: State::default()
    }}
}