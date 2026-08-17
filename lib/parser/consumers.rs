//^
//^ CONSUMERS
//^

//> CONSUMERS -> KEYWORD
#[macro_export]
macro_rules! keyword {
    ($step:ident, []) => {Ok(())};
    ($step:ident, [b'\n' $(, $rest:literal)*]) => {
        match $step.input.get($step.state.position.index) {
            Some(b'\n') => {
                $step.state.position.index += 1;
                $step.state.position.line += 1;
                $step.state.position.column = 0;
                $crate::parser::consumers::keyword!($step, [$($rest),*])
            },
            Some(other) => Err($crate::error::Error::ScanMismatch {
                expected: stringify!(b'\n'),
                encountered: other
            }),
            None => Err($crate::error::Error::TokenStreamDepleted)
        }
    };
    ($step:ident, [$current:literal $(, $rest:literal)*]) => {
        match $step.input.get($step.state.position.index) {
            Some($current) => {
                $step.state.position.index += 1;
                $step.state.position.column += 1;
                $crate::parser::consumers::keyword!($step, [$($rest),*])
            },
            Some(other) => Err($crate::error::Error::ScanMismatch {
                expected: stringify!($current),
                encountered: other
            }),
            None => Err($crate::error::Error::TokenStreamDepleted)
        }
    }
}

//> CONSUMERS -> CHOICE
#[macro_export]
macro_rules! choice {
    ($step:ident, $first:literal, $second:literal) => {
        match $step.input.get($step.state.position.index) {
            Some($first) => {
                $step.state.position.index += 1;
                $step.state.position.column += 1;
                Ok(true)
            },
            Some($second) => {
                $step.state.position.index += 1;
                $step.state.position.column += 1;
                Ok(false)
            },
            Some(other) => Err($crate::error::Error::ScanMismatch {
                expected: stringify!($first | $second),
                encountered: other
            }),
            None => Err($crate::error::Error::TokenStreamDepleted)
        }
    };
}

//> CONSUMERS -> HOOK
#[macro_export]
macro_rules! hook {
    ($step:ident, $pattern:pat, $exceptions:pat) => {{
        let initial = $step.state.position.index;
        while let Some($pattern) = $step.input.get($step.state.position.index) {
            $step.state.position.index += 1;
            $step.state.position.column += 1;
        };
        match &$step.input[initial..$step.state.position.index] {
            found @ $exceptions => Err($crate::error::Error::HookExceptionFound {
                pattern: stringify!($pattern),
                exceptions: stringify!($exceptions),
                found: found
            }),
            other => Ok(other)
        }
    }};
}

//> CONSUMERS -> EXPORTS
pub use keyword;
pub use choice;
pub use hook;