//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    position::Position,
    symbol::Symbol
};

//> HEAD -> STD
use std::collections::HashMap as Map;


//^
//^ STATE
//^

//> STATE -> STRUCT
pub struct State<'valid> {
    pub input: &'valid [u8],
    pub position: Position,
    pub symbols: Map<&'valid [u8], Symbol>
}

//> STATE -> FROM TOKENS
impl<'valid> From<&'valid [u8]> for State<'valid> {
    fn from(value: &'valid [u8]) -> Self {return Self {
        input: value,
        position: Position::default(),
        symbols: Map::default()
    }}
}