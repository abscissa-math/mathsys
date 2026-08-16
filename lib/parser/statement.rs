//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    state::State,
    expression::expression,
    value::identifier,
    keyword::keyword,
    symbol::Symbol,
    optional::optional,
    multiple::multiple
};

//> HEAD -> CRATE
use crate::{
    syntax::statement::{
        Statement,
        Definition,
        Function,
        Node,
        Equation
    },
    error::Error
};


//^
//^ STATEMENT
//^

//> STATEMENT -> DISPATCH
pub fn statement<'input>(
    state: &mut State<'input>
) -> Result<Statement<'input>, Error<'input>> {return match optional!(state, definition) {
    Some(definition) => Ok(Statement::Definition(definition)),
    _ => match optional!(state, function) {
        Some(function) => Ok(Statement::Function(function)),
        _ => match optional!(state, equation) {
            Some(equation) => Ok(Statement::Equation(equation)),
            _ => match optional!(state, node) {
                Some(node) => Ok(Statement::Node(node)),
                _ => Err(Error::CouldntParseStatement)
            }
        }
    }
}}

//> STATEMENT -> DEFINITION
pub fn definition<'input>(
    state: &mut State<'input>
) -> Result<Definition<'input>, Error<'input>> {
    let of = identifier(state)?;
    state.symbols.try_insert(of.name, Symbol::Variable);
    keyword!(state, [b' ', b':', b'=', b' '])?;
    return Ok(Definition {
        of: of,
        expression: expression(state)?
    });
}

//> STATEMENT -> FUNCTION
pub fn function<'input>(
    state: &mut State<'input>
) -> Result<Function<'input>, Error<'input>> {
    let name = identifier(state)?;
    state.symbols.try_insert(name.name, Symbol::Function);
    keyword!(state, [b'('])?;
    let arguments = optional!(state, {
        let mut rest = Vec::from([identifier(state)?]);
        rest.extend(multiple!(state, {
            keyword!(state, [b',', b' '])?;
            identifier(state)
        }));
        Ok(rest)
    }).unwrap_or_default();
    keyword!(state, [b')', b' ', b':', b'=', b' '])?;
    return Ok(Function {
        name: name,
        arguments: arguments,
        expression: expression(state)?
    })
}

//> STATEMENT -> NODE
pub fn node<'input>(
    state: &mut State<'input>
) -> Result<Node<'input>, Error<'input>> {return Ok(Node {
    expression: expression(state)?
})}

//> STATEMENT -> EQUATION
pub fn equation<'input>(
    state: &mut State<'input>
) -> Result<Equation<'input>, Error<'input>> {
    let left = expression(state)?;
    keyword!(state, [b' ', b'=', b' '])?;
    return Ok(Equation {
        expressions: [left, expression(state)?]
    });
}