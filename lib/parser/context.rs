//^
//^ HEAD
//^

//> HEAD -> STD
use std::collections::HashMap as Map;

//> HEAD -> SUPER
use super::symbol::Symbol;

//> HEAD -> CRATE
use crate::Error;


//^
//^ CONTEXT
//^

//> CONTEXT -> STRUCT
#[derive(Default)]
pub struct Context<'valid> {
    symbols: Map<&'valid [u8], Symbol>
}

//> CONTEXT -> IMPLEMENTATION
impl<'valid> Context<'valid> {
    pub fn declare(&mut self, name: &'valid [u8], symbol: Symbol) -> () {
        self.symbols.insert(name, symbol);
    }
    pub fn idcheck(
        &self, 
        name: &'valid [u8], 
        symbol: Symbol
    ) -> Result<(), Error<'static>> {return match self.symbols.get(name) == Some(&symbol) {
        true => Ok(()),
        false => Err(Error::OtherIdentifierSymbolExpected)
    }}
}