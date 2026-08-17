//^ 
//^ HEAD
//^ 

//> HEAD -> SUPER
use super::{
    step::Step,
    expression::expression,
    consumers::{
        hook,
        keyword
    },
    quantifiers::{
        optional,
        multiple
    },
    symbol::Symbol
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
pub fn value<'input>(step: &mut Step<'input>) -> Result<Value<'input>, Error<'input>> {
    return match optional!(@infinite, step) {
        Ok(infinite) => Ok(Value::Infinite(infinite)),
        Err(infinite) => match optional!(@call, step) {
            Ok(call) => Ok(Value::Call(call)),
            Err(call) => match optional!(@nest, step) {
                Ok(nest) => Ok(Value::Nest(nest)),
                Err(nest) => match optional!(@vector, step) {
                    Ok(vector) => Ok(Value::Vector(vector)),
                    Err(vector) => match optional!(@number, step) {
                        Ok(number) => Ok(Value::Number(number)),
                        Err(number) => match optional!(@absolute, step) {
                            Ok(absolute) => Ok(Value::Absolute(absolute)),
                            Err(absolute) => match optional!(@undefined, step) {
                                Ok(undefined) => Ok(Value::Undefined(undefined)),
                                Err(undefined) => match optional!(
                                    @identifier, 
                                    step, 
                                    Symbol::Variable, 
                                    false
                                ) {
                                    Ok(identifier) => Ok(Value::Identifier(identifier)),
                                    Err(identifier) => Err(Error::ParsingValue {
                                        infinite: Box::new(infinite), 
                                        identifier: Box::new(identifier), 
                                        nest: Box::new(nest), 
                                        vector: Box::new(vector), 
                                        number: Box::new(number), 
                                        absolute: Box::new(absolute), 
                                        undefined: Box::new(undefined), 
                                        call: Box::new(call)
                                    })
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
pub fn infinite<'input>(step: &mut Step<'input>) -> Result<Infinite, Error<'input>> {
    keyword!(step, [b'i', b'n', b'f'])?;
    return Ok(Infinite);
}

//> VALUE -> IDENTIFIER
pub fn identifier<'input>(
    step: &mut Step<'input>,
    symbol: Symbol,
    new: bool
) -> Result<Identifier<'input>, Error<'input>> {
    let name = hook!(
        step, 
        b'a'..=b'z' | b'A'..=b'Z' | b'$'..=b'%', 
        b"lim" | b"of" | b"inf" | b""
    )?;
    if new {
        step.state.context.declare(name, symbol)
    } else {
        step.state.context.idcheck(name, symbol)?
    };
    return Ok(Identifier {
        name: name
    });
}

//> VALUE -> NEST
pub fn nest<'input>(step: &mut Step<'input>) -> Result<Nest<'input>, Error<'input>> {
    keyword!(step, [b'('])?;
    let inside = optional!(expression, step);
    keyword!(step, [b')'])?;
    return Ok(Nest {
        inside: inside
    });
}

//> VALUE -> VECTOR
pub fn vector<'input>(step: &mut Step<'input>) -> Result<Vector<'input>, Error<'input>> {
    keyword!(step, [b'['])?;
    let expressions = optional!({
        let mut rest = Vec::from([expression(step)?]);
        rest.extend(multiple!({
            keyword!(step, [b',', b' '])?;
            expression(step)
        }, step));
        Ok(rest)
    }, step).unwrap_or_default();
    keyword!(step, [b']'])?;
    return Ok(Vector {
        expressions: expressions
    });
}

//> VALUE -> NUMBER
pub fn number<'input>(
    step: &mut Step<'input>
) -> Result<Number<'input>, Error<'input>> {return Ok(Number {
    number: hook!(step, b'0'..=b'9', b"")?
})}

//> VALUE -> ABSOLUTE
pub fn absolute<'input>(
    step: &mut Step<'input>
) -> Result<Absolute<'input>, Error<'input>> {
    keyword!(step, [b'|'])?;
    let expression = expression(step)?;
    keyword!(step, [b'|'])?;
    return Ok(Absolute {
        expression: expression
    });
}

//> VALUE -> UNDEFINED
pub fn undefined<'input>(step: &mut Step<'input>) -> Result<Undefined, Error<'input>> {
    keyword!(step, [b'?'])?;
    return Ok(Undefined);
}

//> VALUE -> CALL
pub fn call<'input>(step: &mut Step<'input>) -> Result<Call<'input>, Error<'input>> {
    let identifier = identifier(step, Symbol::Function, false)?;
    keyword!(step, [b'('])?;
    let with = optional!({
        let first = expression(step)?;
        let mut rest = multiple!({
            keyword!(step, [b',', b' '])?;
            expression(step)
        }, step);
        rest.insert(0, first);
        Ok(rest)
    }, step).unwrap_or_default();
    keyword!(step, [b')'])?;
    return Ok(Call {
        identifier: identifier,
        with: with
    });
}