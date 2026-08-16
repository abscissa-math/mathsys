//^
//^ MULTIPLE
//^

//> MULTIPLE -> MACRO
#[macro_export]
macro_rules! multiple {
    ($state:ident, $call:expr) => {{
        let mut items = Vec::new();
        loop {match $crate::parser::optional::optional!($state, $call) {
            Some(value) => items.push(value),
            None => break items
        }}
    }};
}

//> MULTIPLE -> EXPORT
pub use multiple;