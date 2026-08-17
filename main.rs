//^
//^ HEAD
//^

//> HEAD -> FEATURES
#![feature(default_field_values)]
#![feature(const_trait_impl)]
#![feature(never_type)]

//> HEAD -> MODULES
mod error;
mod handler;
mod severities;

//> HEAD -> SYSTEMSTD
use systemstd::{
    System,
    Argument
};

//> HEAD -> ERROR
use error::Error;

//> HEAD -> MATHSYS
use mathsys::Interpreter;

//> HEAD -> HANDLER
use handler::Handler;

//> HEAD -> SEVERITIES
use severities::Process;


//^
//^ MAIN
//^

//> MAIN -> FUNCTION
fn main() -> () {
    let interpreter = Interpreter::from(Handler::default());
    let (target, arguments) = match System::arguments() {
        [Argument::Target {to}, arguments @ ..] => (to, arguments),
        [Argument::Path {..}, Argument::Target {to}, arguments @ ..] => (to, arguments),
        _ => System::error::<Process>(Error::NoTargetProvided)
    };
    match target.as_str() {
        "latex" => {
            System::print(&interpreter.latex(match arguments {
                [Argument::Path {buffer}] => buffer,
                _ => System::error::<Process>(Error::IncorrectLatexArguments)
            }.to_str().unwrap()), false);
        },
        name => System::error::<Process>(Error::UnknownTarget {
            name: name
        })
    };
}