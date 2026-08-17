//^
//^ SCOPE
//^

//> SCOPE -> MACRO
#[macro_export]
macro_rules! scope {
    ($step:ident, $($inside:tt)*) => {
        let context = $step.state.context.clone();
        $($inside)*
        $step.state.context = context;
    };
}

//> SCOPE -> EXPORT
pub use scope;