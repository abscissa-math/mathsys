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


//^
//^ EXPRESSION
//^

//> EXPRESSION -> FUNCTION
pub fn expression<'input>(
    step: &mut Step<'input>
) -> Result<Expression<'input>, Error<'input>> {
    let mut terms = Vec::from([(multiple!(choice!(step, b'+', b'-'), step), term(step)?)]);
    terms.extend(multiple!({
        optional!(keyword!(step, [b' ']), step);
        let signs = more!(choice!(step, b'+', b'-'), step)?;
        if !signs.is_empty() {keyword!(step, [b' '])?}
        Ok((signs, term(step)?))
    }, step));
    return Ok(Expression {
        terms: terms
    });
}