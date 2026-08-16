//^
//^ KEYWORD
//^

//> KEYWORD -> MACRO
#[macro_export]
macro_rules! keyword {
    ($state:ident, []) => {Ok(())};
    ($state:ident, [b'\n' $(, $rest:literal)*]) => {
        match $state.input.get($state.position.index) {
            Some(b'\n') => {
                $state.position.index += 1;
                $state.position.line += 1;
                $state.position.column = 0;
                keyword!($state, [$($rest),*])
            },
            Some(other) => Err($crate::error::Error::InputParseFailed {
                expected: stringify!(b'\n'),
                encountered: other
            }),
            None => Err($crate::error::Error::TokenStreamDepleted)
        }
    };
    ($state:ident, [$current:literal $(, $rest:literal)*]) => {
        match $state.input.get($state.position.index) {
            Some($current) => {
                $state.position.index += 1;
                $state.position.column += 1;
                keyword!($state, [$($rest),*])
            },
            Some(other) => Err($crate::error::Error::InputParseFailed {
                expected: stringify!($current),
                encountered: other
            }),
            None => Err($crate::error::Error::TokenStreamDepleted)
        }
    }
}

//> KEYWORD -> EXPORT
pub use keyword;