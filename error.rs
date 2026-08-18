//^
//^ HEAD
//^

//> HEAD -> ISSUING
use issuing::{
    Issue,
    Section
};

//> HEAD -> SYSTEMSTD
use systemstd::Severity;

//> HEAD -> STD
use std::panic::set_hook;


//>
//^ ERROR
//^

//> ERROR -> ENUM
pub enum Error {
    UnknownTarget {
        name: &'static str
    },
    NoTargetProvided,
    IncorrectLatexArguments
}

//> ERROR -> INTO ISSUE
impl Into<Issue> for Error {
    fn into(self) -> Issue {return match self {
        Error::UnknownTarget {name} => Issue {
            name: "unknown target",
            sections: Vec::from([
                Section::Code {
                    extends: Box::new(Section::Cause(format!("unknown target provided"))),
                    code: name.to_string(), 
                    ..
                }
            ]),
            ..
        },
        Error::NoTargetProvided => Issue {
            name: "target not provided",
            sections: Vec::from([
                Section::Cause(format!("interpreter target was not provided"))
            ]),
            ..
        },
        Error::IncorrectLatexArguments => Issue {
            name: "incorrect arguments for latex",
            sections: Vec::from([
                Section::Help(format!("usage: `mathsys latex FILE`"))
            ]),
            ..
        }
    }}
}

//> ERROR -> SEVERITY
impl Severity for Error {
    type Then = !;
    fn done() -> Self::Then {
        set_hook(Box::new(|_| ()));
        panic!();
    }
}