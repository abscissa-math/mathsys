//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    step::Step,
    statement::statement,
    consumers::keyword,
    quantifiers::multiple,
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
pub fn start<'input>(step: &mut Step<'input>) -> Result<Start<'input>, Error<'input>> {
    let (statements, error) = multiple!(@{
        if step.state.position.index != 0 {keyword!(step, [b'\n'])?}
        statement(step)
    }, step);
    depleted!(step, error)?;
    return Ok(Start {
        statements: statements
    });
}