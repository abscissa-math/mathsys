//^
//^ HEAD
//^

//> HEAD -> ISSUING
use issuing::{
    Issue,
    Section,
    Span
};


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
            deprecation: Some(format!("unknown target provided")),
            sections: Vec::from([
                Section::Code {
                    code: name.to_string(), 
                    message: Some(format!("unknown target")), 
                    span: Some(Span::RangeFull(..)), 
                    ..
                }
            ]),
            description: Some(format!("unknown target found: {name:?}")),
            ..
        },
        Error::NoTargetProvided => Issue {
            name: "target not provided",
            description: Some(String::from("interpreter target was not provided")),
            ..
        },
        Error::IncorrectLatexArguments => Issue {
            name: "incorrect arguments for latex",
            description: Some(format!("usage: `mathsys latex FILE`")),
            ..
        }
    }}
}