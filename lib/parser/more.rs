//^
//^ MORE
//^

//> MORE -> MACRO
#[macro_export]
macro_rules! more {
    ($state:ident, $call:expr) => {{
        let items = $crate::parser::multiple::multiple!($state, $call);
        match items.len() {
            1.. => Ok(items),
            0 => Err($crate::error::Error::CouldntParseMore)
        }
    }};
}

//> MORE -> EXPORT
pub use more;