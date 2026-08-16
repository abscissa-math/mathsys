//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    state::State,
    term::term,
    keyword::keyword,
    choice::choice,
    optional::optional,
    multiple::multiple,
    more::more
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
    state: &mut State<'input>
) -> Result<Expression<'input>, Error<'input>> {
    let mut terms = Vec::from([(multiple!(state, choice!(state, b'+', b'-')), term(state)?)]);
    terms.extend(multiple!(state, {
        optional!(state, keyword!(state, [b' ']));
        let signs = more!(state, choice!(state, b'+', b'-'))?;
        if !signs.is_empty() {keyword!(state, [b' '])?}
        Ok((signs, term(state)?))
    }));
    return Ok(Expression {
        terms: terms
    });
}