//^
//^ HEAD
//^

//> HEAD -> ENUM_AS_INNER
use enum_as_inner::EnumAsInner;


//^
//^ SYMBOL
//^

//> SYMBOL -> ENUM
#[derive(Clone, EnumAsInner, PartialEq, Eq)]
pub enum Symbol {
    Variable,
    Function
}