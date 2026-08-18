//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::term::Term;

//> HEAD -> NONEMPTY
use nonempty::NonEmpty;


//^
//^ EXPRESSION
//^

//> EXPRESSION -> STRUCT
pub struct Expression<'valid> {
    pub terms: NonEmpty<(Vec<bool>, Term<'valid>)>
}