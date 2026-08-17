//^
//^ HEAD
//^

//> HEAD -> MODULES
pub mod coerce;
pub mod consumers;
pub mod context;
pub mod expression;
pub mod factor;
pub mod position;
pub mod quantifiers;
pub mod scope;
pub mod start;
pub mod state;
pub mod statement;
pub mod step;
pub mod symbol;
pub mod term;
pub mod value;

//> HEAD -> CRATE
use crate::{
    syntax::Start,
    runtime::Runtime
};

//> HEAD -> STEP
use step::Step;

//> HEAD -> START
use start::start;


//^
//^ PARSER
//^

//> PARSER -> FUNCTION6
pub fn parse<'input, Implementation: Runtime<'input>>(
    input: &'input [u8]
) -> Start<'input> {return match start(&mut Step::from(input)) {
    Ok(start) => start,
    Err(error) => Implementation::failure(error)
}}