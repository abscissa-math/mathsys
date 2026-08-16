//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    state::State,
    statement::statement,
    keyword::keyword,
    multiple::multiple,
    depleted::depleted
};

//> HEAD -> CRATE
use crate::{
    error::Error,
    syntax::Start
};


//^
//^ START
//^

//> START -> FUNCTION
pub fn start<'input>(state: &mut State<'input>) -> Result<Start<'input>, Error<'input>> {
    let statements = multiple!(state, {
        if state.position.index != 0 {keyword!(state, [b'\n'])?}
        statement(state)
    });
    depleted!(state)?;
    return Ok(Start {
        statements: statements
    });
}