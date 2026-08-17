//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    position::Position,
    context::Context
};


//^
//^ STATE
//^

//> STATE -> STRUCT
#[derive(Default, Clone)]
pub struct State<'valid> {
    pub position: Position,
    pub context: Context<'valid>
}