//^
//^ OPTIONAL
//^

//> OPTIONAL -> MACRO
#[macro_export]
macro_rules! optional {
    ($state:ident, $call:ident) => {{
        let $crate::parser::checkpoint::Checkpoint {
            position,
            symbols
        } = $crate::parser::checkpoint::Checkpoint::from(&*$state);
        match $call($state) {
            Ok(value) => Some(value),
            Err(_) => {
                $state.position = position;
                $state.symbols = symbols;
                None
            }
        }
    }};
    ($state:ident, $closure:expr) => {{
        let closure = $crate::parser::check::check(
            |state| unhygienic2::unhygienic! {$closure}
        );
        $crate::parser::optional::optional!($state, closure)
    }}
}

//> OPTIONAL -> EXPORT
pub use optional;