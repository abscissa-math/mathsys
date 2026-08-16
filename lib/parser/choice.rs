//^
//^ CHOICE
//^

//> CHOICE -> MACRO
#[macro_export]
macro_rules! choice {
    ($state:ident, $first:literal, $second:literal) => {
        match $state.input.get($state.position.index) {
            Some($first) => {
                $state.position.index += 1;
                $state.position.column += 1;
                Ok(true)
            },
            Some($second) => {
                $state.position.index += 1;
                $state.position.column += 1;
                Ok(false)
            },
            Some(other) => Err($crate::error::Error::InputParseFailed {
                expected: stringify!($first | $second),
                encountered: other
            }),
            None => Err($crate::error::Error::TokenStreamDepleted)
        }
    };
}

//> CHOICE -> EXPORT
pub use choice;