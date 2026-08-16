//^
//^ HEAD
//^

//> HEAD -> ISSUING
use issuing::Issue;


//>
//^ FAILURE
//^

//> FAILURE -> ENUM
pub enum Failure {
    UnknownTarget {
        name: &'static str
    },
    TargetNotProvided,
    IncorrectLatexArguments
}

//> FAILURE -> INTO ISSUE
impl Into<Issue> for Failure {
    fn into(self) -> Issue {return match self {
        Failure::UnknownTarget {name} => Issue {
            name: "unknown target",
            description: Some(format!("unknown target found: {name:?}")),
            ..
        },
        Failure::TargetNotProvided => Issue {
            name: "target not provided",
            description: Some(String::from("interpreter target was not provided")),
            ..
        },
        Failure::IncorrectLatexArguments => Issue {
            name: "incorrect arguments for latex",
            description: Some(format!("usage: `mathsys latex (FILE)`")),
            ..
        }
    }}
}