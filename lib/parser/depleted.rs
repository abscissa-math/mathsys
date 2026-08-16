//^
//^ DEPLETED
//^

//> DEPLETED -> MACRO
#[macro_export]
macro_rules! depleted {
    ($state:ident) => {if $state.input.len() == $state.position.index {Ok(())} else {
        Err($crate::error::Error::UnfinishedInputParse)
    }};
}


//> DEPLETED -> EXPORT
pub use depleted;