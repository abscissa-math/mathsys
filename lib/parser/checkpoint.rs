//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    position::Position,
    symbol::Symbol,
    state::State
};

//> HEAD -> STD
use std::collections::HashMap as Map;


//^
//^ CHECKPOINT
//^

//> CHECKPOINT -> STRUCT
pub struct Checkpoint<'valid> {
    pub position: Position,
    pub symbols: Map<&'valid [u8], Symbol>
}

//> CHECKPOINT -> FROM MUTABLE STATE
impl<'valid> From<&State<'valid>> for Checkpoint<'valid> {
    fn from(value: &State<'valid>) -> Self {return Self {
        position: value.position.clone(),
        symbols: value.symbols.clone()
    }}
}