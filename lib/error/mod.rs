//^
//^ HEAD
//^

//> HEAD -> ISSUING
use issuing::{
    Issue,
    Section
};

//> HEAD -> CRATE
use crate::parser::symbol::Symbol;

//> HEAD -> SYSTEMSTD
use systemstd::Severity;

//> HEAD -> STD
use std::panic::set_hook;


//^
//^ ERROR
//^

//> ERROR -> ENUM
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
    OtherIdentifierSymbolExpected {
        name: &'valid [u8],
        expected: Symbol
    },
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
    TokenStreamDepleted {
        expected: &'static str
    },
    CouldntParseMore
}

//> ERROR -> INTO ISSUE
impl<'valid> Into<Issue> for Error<'valid> {
    fn into(self) -> Issue {return match self {
        Error::ScanMismatch {expected, encountered} => Issue {
            name: "mismatched input",
            sections: Vec::from([
                Section::Cause(format!(
                    "expected {expected} but found b{:?}", 
                    unsafe {char::from_u32_unchecked(*encountered as u32)}
                )),
                Section::Help(format!("write {expected} instead"))
            ]),
            ..
        },
        Error::HookExceptionFound {pattern, exceptions, found} => Issue {
            name: "exception found whilst parsing input",
            sections: Vec::from([
                Section::Cause(format!(
                    "cannot parse the exception b{:?}",
                    unsafe {str::from_utf8_unchecked(found)}
                )),
                Section::Help(format!("the sequence must not be {exceptions}")),
                Section::Note(format!("write the sequence with {pattern}"))
            ]),
            ..
        },
        Error::OtherIdentifierSymbolExpected {name, expected} => Issue {
            name: "unmatched identifier symbol",
            sections: Vec::from([
                Section::Cause(format!(
                    "expected b{:?} to be a {expected:?}",
                    unsafe {str::from_utf8_unchecked(name)}
                )),
                Section::Code {
                    extends: Box::new(Section::Help(format!(
                        "you might want to define b{:?} as a {expected:?}:",
                        unsafe {str::from_utf8_unchecked(name)}
                    ))),
                    code: format!(
                        "{}{} := //..//",
                        unsafe {str::from_utf8_unchecked(name)},
                        match expected {
                            Symbol::Function => "(//..//)",
                            Symbol::Variable => ""
                        }
                    ),
                    ..
                }
            ]),
            ..
        },
        Error::ParsingStatement {definition, function, node, equation} => Issue {
            name: "failed to parse statement",
            sections: Vec::from([
                Section::Cause(format!("failed to parse any statement construct")),
                Section::Note(String::from("attempted to parse the following constructs")),
                Section::Help(format!("for parsing a definition:")),
                Section::Child((*definition).into()),
                Section::Help(format!("for parsing a function:")),
                Section::Child((*function).into()),
                Section::Help(format!("for parsing a node:")),
                Section::Child((*node).into()),
                Section::Help(format!("for parsing an equation:")),
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
                Section::Cause(format!("failed to parse any value construct")),
                Section::Note(String::from("attempted to parse the following constructs")),
                Section::Help(format!("for parsing an infinite:")),
                Section::Child((*infinite).into()),
                Section::Help(format!("for parsing an identifier:")),
                Section::Child((*identifier).into()),
                Section::Help(format!("for parsing a nest:")),
                Section::Child((*nest).into()),
                Section::Help(format!("for parsing a vector:")),
                Section::Child((*vector).into()),
                Section::Help(format!("for parsing a number:")),
                Section::Child((*number).into()),
                Section::Help(format!("for parsing an absolute:")),
                Section::Child((*absolute).into()),
                Section::Help(format!("for parsing an undefined:")),
                Section::Child((*undefined).into()),
                Section::Help(format!("for parsing a call:")),
                Section::Child((*call).into())
            ]),
            ..
        },
        Error::TokenStreamDepleted {expected} => Issue {
            name: "unexpected end of file",
            sections: Vec::from([
                Section::Cause(format!("unexpectedly encountered the end of file")),
                Section::Note(format!("expected {expected} instead"))
            ]),
            ..
        },
        Error::CouldntParseMore => Issue {
            name: "failed to parse more",
            ..
        }
    }}
}

//> ERROR -> SEVERITY
impl<'valid> Severity for Error<'valid> {
    type Then = !;
    fn done() -> Self::Then {
        set_hook(Box::new(|_| ()));
        panic!();
    }
}