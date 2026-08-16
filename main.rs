//^
//^ HEAD
//^

//> HEAD -> FEATURES
#![feature(default_field_values)]
#![feature(const_trait_impl)]

//> HEAD -> MODULES
mod failure;
mod handler;

//> HEAD -> SYSTEMSTD
use systemstd::{
    System,
    Argument
};

//> HEAD -> FAILURE
use failure::Failure;

//> HEAD -> MATHSYS
use mathsys::Interpreter;

//> HEAD -> HANDLER
use handler::Handler;


//^
//^ MAIN
//^

//> MAIN -> FUNCTION
fn main() -> () {
    let interpreter = Interpreter::from(Handler::default());
    let (target, arguments) = match System::arguments() {
        [Argument::Target {to}, arguments @ ..] => (to, arguments),
        [Argument::Path {..}, Argument::Target {to}, arguments @ ..] => (to, arguments),
        _ => System::critical([Failure::TargetNotProvided])
    };
    System::print(match target.as_str() {
        "latex" => {
            let file = match arguments {
                [Argument::Path {buffer}] => buffer,
                _ => System::critical([Failure::IncorrectLatexArguments])
            };
            interpreter.latex(file.to_str().unwrap())
        },
        name => System::critical([Failure::UnknownTarget {
            name: name
        }])
    });
}