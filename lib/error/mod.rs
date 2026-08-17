//^
//^ HEAD
//^

//> HEAD -> ISSUING
use issuing::{
    Issue,
    Section
};

//> HEAD -> ENUM_AS_INNER
use enum_as_inner::EnumAsInner;


//^
//^ ERROR
//^

//> ERROR -> ENUM
#[derive(EnumAsInner)]
pub enum Error<'valid> {
    ScanMismatch {
        expected: &'static str,
        encountered: &'valid u8
    },
    HookExceptionFound {
        pattern: &'static str,
        exceptions: &'static str,
        found: &'valid [u8]
    },
    OtherIdentifierSymbolExpected,
    ParsingStatement {
        definition: Box<Error<'valid>>,
        function: Box<Error<'valid>>,
        node: Box<Error<'valid>>,
        equation: Box<Error<'valid>>
    },
    ParsingValue {
        infinite: Box<Error<'valid>>,
        identifier: Box<Error<'valid>>,
        nest: Box<Error<'valid>>,
        vector: Box<Error<'valid>>,
        number: Box<Error<'valid>>,
        absolute: Box<Error<'valid>>,
        undefined: Box<Error<'valid>>,
        call: Box<Error<'valid>>
    },
    TokenStreamDepleted,
    CouldntParseMore
}

//> ERROR -> INTO ISSUE
impl<'valid> Into<Issue> for Error<'valid> {
    fn into(self) -> Issue {return match self {
        Error::ScanMismatch {expected, encountered} => Issue {
            name: "failed to parse input",
            description: Some(format!(
                "expected {expected} but found b{:?}", 
                unsafe {char::from_u32_unchecked(*encountered as u32)}
            )),
            sections: Vec::from([
                Section::Help(format!("write {expected} instead"))
            ]),
            ..
        },
        Error::HookExceptionFound {pattern, exceptions, found} => Issue {
            name: "failed to hook into the document",
            description: Some(format!("hit an exception of {exceptions}")),
            sections: Vec::from([
                Section::Help(format!("write a sequence of {pattern} not in {exceptions}"))
            ]),
            ..
        },
        Error::OtherIdentifierSymbolExpected => Issue {
            name: "other",
            ..
        },
        Error::ParsingStatement {definition, function, node, equation} => Issue {
            name: "failed to parse statement",
            sections: Vec::from([
                Section::Child((*definition).into()),
                Section::Child((*function).into()),
                Section::Child((*node).into()),
                Section::Child((*equation).into())
            ]),
            ..
        },
        Error::ParsingValue {
            infinite, 
            identifier, 
            nest, 
            vector, 
            number, 
            absolute, 
            undefined, 
            call
        } => Issue {
            name: "failed to parse value",
            sections: Vec::from([
                Section::Child((*infinite).into()),
                Section::Child((*identifier).into()),
                Section::Child((*nest).into()),
                Section::Child((*vector).into()),
                Section::Child((*number).into()),
                Section::Child((*absolute).into()),
                Section::Child((*undefined).into()),
                Section::Child((*call).into())
            ]),
            ..
        },
        Error::TokenStreamDepleted => Issue {
            name: "token stream depleted",
            ..
        },
        Error::CouldntParseMore => Issue {
            name: "failed to parse more",
            ..
        }
    }}
}