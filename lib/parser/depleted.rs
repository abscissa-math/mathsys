//^
//^ DEPLETED
//^

//> DEPLETED -> MACRO
#[macro_export]
macro_rules! depleted {
    ($step:ident, $error:ident) => {
        if $step.input.len() == $step.state.position.index {Ok(())} else {Err($error)}
    };
}

//> DEPLETED -> EXPORT
pub use depleted;