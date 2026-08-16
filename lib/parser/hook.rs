//^
//^ HOOK
//^

//> HOOK -> MACRO
#[macro_export]
macro_rules! hook {
    ($state:ident, $pattern:pat, $exceptions:pat) => {{
        let initial = $state.position.index;
        while let Some($pattern) = $state.input.get($state.position.index) {
            $state.position.index += 1;
            $state.position.column += 1;
        };
        match &$state.input[initial..$state.position.index] {
            found @ $exceptions => Err($crate::error::Error::HookException {
                pattern: stringify!($pattern),
                exceptions: stringify!($exceptions),
                found: found
            }),
            other => Ok(other)
        }
    }};
}

//> HOOK -> EXPORT
pub use hook;