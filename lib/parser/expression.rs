//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    step::Step,
    term::term,
    consumers::{
        keyword,
        choice
    },
    quantifiers::{
        optional,
        multiple,
        more
    }
};

//> HEAD -> CRATE
use crate::{
    syntax::expression::Expression,
    error::Error
};

//> HEAD -> NONEMPTY
use nonempty::NonEmpty;


//^
//^ EXPRESSION
//^

//> EXPRESSION -> FUNCTION
pub fn expression<'input>(
    step: &mut Step<'input>
) -> Result<Expression<'input>, Error<'input>> {
    let first = (multiple!(choice!(step, b'+', b'-'), step), term(step)?);
    let rest = multiple!({
        optional!(keyword!(step, [b' ']), step);
        let signs = Vec::from(more!(choice!(step, b'+', b'-'), step)?);
        keyword!(step, [b' '])?;
        Ok((signs, term(step)?))
    }, step);
    return Ok(Expression {
        terms: NonEmpty::from((first, rest))
    });
}