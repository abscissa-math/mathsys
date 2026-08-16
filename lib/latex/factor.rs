//^
//^ HEAD
//^

//> HEAD -> CRATE
use crate::syntax::factor::Factor;

//> HEAD -> SUPER
use super::LaTeX;


//^
//^ FACTOR
//^

//> FACTOR -> IMPLEMENTATION
impl<'valid> LaTeX for Factor<'valid> {
    fn render(&self) -> String {return format!(
        "{}{}", 
        self.value.render(), 
        self.exponent.as_ref().map(|exponent| format!(
            "^{{{}}}", 
            exponent.render()
        )).unwrap_or_default()
    )}
}