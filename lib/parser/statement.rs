//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    step::Step,
    expression::expression,
    value::identifier,
    consumers::keyword,
    quantifiers::{
        optional,
        multiple
    },
    symbol::Symbol,
    scope::scope
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
    step: &mut Step<'input>
) -> Result<Statement<'input>, Error<'input>> {return match optional!(@definition, step) {
    Ok(definition) => Ok(Statement::Definition(definition)),
    Err(definition) => match optional!(@function, step) {
        Ok(function) => Ok(Statement::Function(function)),
        Err(function) => match optional!(@equation, step) {
            Ok(equation) => Ok(Statement::Equation(equation)),
            Err(equation) => match optional!(@node, step) {
                Ok(node) => Ok(Statement::Node(node)),
                Err(node) => Err(Error::ParsingStatement {
                    definition: Box::new(definition),
                    function: Box::new(function),
                    node: Box::new(node),
                    equation: Box::new(equation)
                })
            }
        }
    }
}}

//> STATEMENT -> DEFINITION
pub fn definition<'input>(
    step: &mut Step<'input>
) -> Result<Definition<'input>, Error<'input>> {
    let identifier = identifier(step, Symbol::Variable, true)?;
    keyword!(step, [b' ', b':', b'=', b' '])?;
    return Ok(Definition {
        identifier: identifier,
        expression: expression(step)?
    });
}

//> STATEMENT -> FUNCTION
pub fn function<'input>(
    step: &mut Step<'input>
) -> Result<Function<'input>, Error<'input>> {
    let name = identifier(step, Symbol::Function, true)?;
    keyword!(step, [b'('])?;
    scope!(
        step,
        let arguments = optional!({
            let mut rest = Vec::from([identifier(step, Symbol::Variable, true)?]);
            rest.extend(multiple!({
                keyword!(step, [b',', b' '])?;
                identifier(step, Symbol::Variable, true)
            }, step));
            Ok(rest)
        }, step).unwrap_or_default();
        keyword!(step, [b')', b' ', b':', b'=', b' '])?;
        let expression = expression(step)?;
    );
    return Ok(Function {
        identifier: name,
        arguments: arguments,
        expression: expression
    })
}

//> STATEMENT -> NODE
pub fn node<'input>(
    step: &mut Step<'input>
) -> Result<Node<'input>, Error<'input>> {return Ok(Node {
    expression: expression(step)?
})}

//> STATEMENT -> EQUATION
pub fn equation<'input>(
    step: &mut Step<'input>
) -> Result<Equation<'input>, Error<'input>> {
    let left = expression(step)?;
    keyword!(step, [b' ', b'=', b' '])?;
    return Ok(Equation {
        expressions: [left, expression(step)?]
    });
}