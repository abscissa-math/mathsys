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
    //($step:ident, @$call:ident) => {{
    //    let state = $step.state.clone();
    //    let result = $call($step);
    //    if result.is_err() {$step.state = state};
    //    result
    //}};
    ($step:ident, $call:ident) => {{
        let state = $step.state.clone();
        match $call($step) {
            Ok(value) => Some(value),
            Err(_) => {
                $step.state = state;
                None
            }
        }
    }};
    ($step:ident, @$closure:expr) => {{
        let closure = $crate::parser::coerce::coerce(
            |step| unhygienic2::unhygienic! {$closure}
        );
        $crate::parser::quantifiers::optional!($step, @closure)
    }};
    ($step:ident, $closure:expr) => {{
        let closure = $crate::parser::coerce::coerce(
            |step| unhygienic2::unhygienic! {$closure}
        );
        $crate::parser::quantifiers::optional!($step, closure)
    }}
}

//> QUANTIFIERS -> MULTIPLE
#[macro_export]
macro_rules! multiple {
    ($step:ident, $call:expr) => {{
        let mut items = Vec::new();
        loop {match $crate::parser::quantifiers::optional!($step, $call) {
            Some(value) => items.push(value),
            None => break items
        }}
    }};
    ($step:ident, @$call:expr) => {{
        let mut items = Vec::new();
        loop {match $crate::parser::quantifiers::optional!($step, @$call) {
            Ok(value) => items.push(value),
            Err(error) => break (items, error)
        }}
    }};
}

//> QUANTIFIERS -> MORE
#[macro_export]
macro_rules! more {
    ($step:ident, $call:expr) => {{
        let items = $crate::parser::quantifiers::multiple!($step, $call);
        match items.len() {
            1.. => Ok(items),
            0 => Err($crate::error::Error::CouldntParseMore)
        }
    }};
    ($step:ident, @$call:expr) => {{
        let items = $crate::parser::quantifiers::multiple!($step, @$call);
        match items.0.len() {
            1.. => Ok(items),
            0 => Err($crate::error::Error::CouldntParseMore)
        }
    }};
}

//> QUANTIFIERS -> EXPORTS
pub use optional;
pub use multiple;
pub use more;