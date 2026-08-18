//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::factor::Factor;

//> HEAD -> NONEMPTY
use nonempty::NonEmpty;


//^
//^ TERM
//^

//> TERM -> STRUCT
pub struct Term<'valid> {
    pub numerator: Box<NonEmpty<Factor<'valid>>>,
    pub denominator: Vec<Factor<'valid>>
}