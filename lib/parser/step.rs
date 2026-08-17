//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::state::State;


//^
//^ STEP
//^

//> STEP -> STRUCT
pub struct Step<'valid> {
    pub input: &'valid [u8],
    pub state: State<'valid>
}

//> STEP -> FROM TOKENS
impl<'valid> From<&'valid [u8]> for Step<'valid> {
    fn from(value: &'valid [u8]) -> Self {return Self {
        input: value,
        state: State::default()
    }}
}