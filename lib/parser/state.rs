//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    position::Position,
    symbol::Symbol,
    context::Context
};

//> HEAD -> STD
use std::collections::HashMap as Map;

//> HEAD -> CRATE
use crate::{
    syntax::value::Identifier,
    error::Error
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