//^
//^ HEAD
//^

//> HEAD -> ISSUING
use issuing::Issue;

//> HEAD -> ENUM_AS_INNER
use enum_as_inner::EnumAsInner;


//^
//^ ERROR
//^

//> ERROR -> ENUM
#[derive(EnumAsInner)]
pub enum Error<'valid> {
    InputParseFailed {
        expected: &'static str,
        encountered: &'valid u8
    },
    HookException {
        pattern: &'static str,
        exceptions: &'static str,
        found: &'valid [u8]
    },
    CouldntParseStatement,
    CouldntParseFactor,
    CouldntParseValue,
    TokenStreamDepleted,
    CouldntParseMore,
    UnfinishedInputParse
}

//> ERROR -> INTO ISSUE
impl<'valid> Into<Issue> for Error<'valid> {
    fn into(self) -> Issue {return match self {
        Error::InputParseFailed {expected, encountered} => Issue {
            name: "failed to parse input",
            description: Some(format!("expected {expected} but found {encountered}")),
            help: Some(format!("write one of {expected} instead")),
            ..
        },
        Error::HookException {pattern, exceptions, found} => Issue {
            name: "failed to hook into the document",
            description: Some(format!("hit an exception of {exceptions}")),
            help: Some(format!("write a sequence of {pattern} not in {exceptions}")),
            ..
        },
        Error::CouldntParseStatement => Issue {
            name: "failed to parse statement",
            ..
        },
        Error::CouldntParseFactor => Issue {
            name: "failed to parse factor",
            ..
        },
        Error::CouldntParseValue => Issue {
            name: "failed to parse value",
            ..
        },
        Error::TokenStreamDepleted => Issue {
            name: "token stream depleted",
            ..
        },
        Error::CouldntParseMore => Issue {
            name: "failed to parse more",
            ..
        },
        Error::UnfinishedInputParse => Issue {
            name: "failed to parse whole input",
            ..
        }
    }}
}