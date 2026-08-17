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
    let mut terms = Vec::from([(multiple!(step, choice!(step, b'+', b'-')), term(step)?)]);
    terms.extend(multiple!(step, {
        optional!(step, keyword!(step, [b' ']));
        let signs = more!(step, choice!(step, b'+', b'-'))?;
        if !signs.is_empty() {keyword!(step, [b' '])?}
        Ok((signs, term(step)?))
    }));
    return Ok(Expression {
        terms: terms
    });
}