//^ 
//^ HEAD
//^ 

//> HEAD -> SUPER
use super::{
    state::State,
    expression::expression,
    keyword::keyword,
    hook::hook,
    symbol::Symbol,
    optional::optional,
    multiple::multiple
};

//> HEAD -> SYNTAX
use crate::{
    syntax::value::{
        Value,
        Infinite,
        Identifier,
        Nest,
        Vector,
        Number,
        Absolute,
        Undefined,
        Call
    },
    error::Error
};


//^
//^ VALUE
//^

//> VALUE -> DISPATCH
pub fn value<'input>(state: &mut State<'input>) -> Result<Value<'input>, Error<'input>> {
    return match optional!(state, infinite) {
        Some(infinite) => Ok(Value::Infinite(infinite)),
        _ => match optional!(state, call) {
            Some(call) if state.symbols.get(
                call.identifier.name
            ).is_some_and(Symbol::is_function) => Ok(Value::Call(call)),
            _ => match optional!(state, nest) {
                Some(nest) => Ok(Value::Nest(nest)),
                _ => match optional!(state, vector) {
                    Some(vector) => Ok(Value::Vector(vector)),
                    _ => match optional!(state, number) {
                        Some(number) => Ok(Value::Number(number)),
                        _ => match optional!(state, absolute) {
                            Some(absolute) => Ok(Value::Absolute(absolute)),
                            _ => match optional!(state, undefined) {
                                Some(undefined) => Ok(Value::Undefined(undefined)),
                                _ => match optional!(state, identifier) {
                                    Some(identifier) => Ok(Value::Identifier(identifier)),
                                    _ => Err(Error::CouldntParseValue)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

//> VALUE -> INFINITE
pub fn infinite<'input>(state: &mut State<'input>) -> Result<Infinite, Error<'input>> {
    keyword!(state, [b'i', b'n', b'f'])?;
    return Ok(Infinite);
}

//> VALUE -> IDENTIFIER
pub fn identifier<'input>(
    state: &mut State<'input>
) -> Result<Identifier<'input>, Error<'input>> {return Ok(Identifier {
    name: hook!(
        state, 
        b'a'..=b'z' | b'A'..=b'Z' | b'$'..=b'%', 
        b"lim" | b"of" | b"inf" | b""
    )?
})}

//> VALUE -> NEST
pub fn nest<'input>(state: &mut State<'input>) -> Result<Nest<'input>, Error<'input>> {
    keyword!(state, [b'('])?;
    let inside = optional!(state, expression);
    keyword!(state, [b')'])?;
    return Ok(Nest {
        inside: inside
    });
}

//> VALUE -> VECTOR
pub fn vector<'input>(state: &mut State<'input>) -> Result<Vector<'input>, Error<'input>> {
    keyword!(state, [b'['])?;
    let expressions = optional!(state, {
        let mut rest = Vec::from([expression(state)?]);
        rest.extend(multiple!(state, {
            keyword!(state, [b',', b' '])?;
            expression(state)
        }));
        Ok(rest)
    }).unwrap_or_default();
    keyword!(state, [b']'])?;
    return Ok(Vector {
        expressions: expressions
    });
}

//> VALUE -> NUMBER
pub fn number<'input>(
    state: &mut State<'input>
) -> Result<Number<'input>, Error<'input>> {return Ok(Number {
    number: hook!(state, b'0'..=b'9', b"")?
})}

//> VALUE -> ABSOLUTE
pub fn absolute<'input>(
    state: &mut State<'input>
) -> Result<Absolute<'input>, Error<'input>> {
    keyword!(state, [b'|'])?;
    let expression = expression(state)?;
    keyword!(state, [b'|'])?;
    return Ok(Absolute {
        expression: expression
    });
}

//> VALUE -> UNDEFINED
pub fn undefined<'input>(state: &mut State<'input>) -> Result<Undefined, Error<'input>> {
    keyword!(state, [b'?'])?;
    return Ok(Undefined);
}

//> VALUE -> CALL
pub fn call<'input>(state: &mut State<'input>) -> Result<Call<'input>, Error<'input>> {
    let identifier = identifier(state)?;
    keyword!(state, [b'('])?;
    let with = optional!(state, {
        let first = expression(state)?;
        let mut rest = multiple!(state, {
            keyword!(state, [b',', b' '])?;
            expression(state)
        });
        rest.insert(0, first);
        Ok(rest)
    }).unwrap_or_default();
    keyword!(state, [b')'])?;
    return Ok(Call {
        identifier: identifier,
        with: with
    });
}