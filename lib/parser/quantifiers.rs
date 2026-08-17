//^
//^ QUANTIFIERS
//^

//> QUANTIFIERS -> OPTIONAL
#[macro_export]
macro_rules! optional {
    (@$call:ident, $step:ident $(, $argument:expr)*) => {{
        let state = $step.state.clone();
        let result = $call($step $(, $argument)*);
        if result.is_err() {$step.state = state};
        result
    }};
    ($call:ident, $step:ident $(, $argument:expr)*) => {{
        let state = $step.state.clone();
        match $call($step, $(, $argument)*) {
            Ok(value) => Some(value),
            Err(_) => {
                $step.state = state;
                None
            }
        }
    }};
    (@$closure:expr, $step:ident $(, $argument:expr)*) => {{
        let closure = $crate::parser::coerce::coerce(
            |step| unhygienic2::unhygienic! {$closure}
        );
        $crate::parser::quantifiers::optional!(@closure, $step $(, $argument)*)
    }};
    ($closure:expr, $step:ident $(, $argument:expr)*) => {{
        let closure = $crate::parser::coerce::coerce(
            |step| unhygienic2::unhygienic! {$closure}
        );
        $crate::parser::quantifiers::optional!(closure, $step $(, $argument)*)
    }}
}

//> QUANTIFIERS -> MULTIPLE
#[macro_export]
macro_rules! multiple {
    ($call:expr, $step:ident $(, $argument:expr)*) => {{
        let mut items = Vec::new();
        loop {match $crate::parser::quantifiers::optional!($call, $step $(, $argument)*) {
            Some(value) => items.push(value),
            None => break items
        }}
    }};
    (@$call:expr, $step:ident $(, $argument:expr)*) => {{
        let mut items = Vec::new();
        loop {match $crate::parser::quantifiers::optional!(@$call, $step $(, $argument)*) {
            Ok(value) => items.push(value),
            Err(error) => break (items, error)
        }}
    }};
}

//> QUANTIFIERS -> MORE
#[macro_export]
macro_rules! more {
    ($call:expr, $step:ident $(, $argument:expr)*) => {{
        let items = $crate::parser::quantifiers::multiple!($call, $step $(, $argument)*);
        match items.len() {
            1.. => Ok(items),
            0 => Err($crate::error::Error::CouldntParseMore)
        }
    }};
    (@$call:expr, $step:ident $(, $argument:expr)*) => {{
        let items = $crate::parser::quantifiers::multiple!(@$call, $step $(, $argument)*);
        match items.0.len() {
            1.. => Ok(items),
            0 => Err($crate::error::Error::CouldntParseMore)
        }
    }}
}

//> QUANTIFIERS -> EXPORTS
pub use optional;
pub use multiple;
pub use more;