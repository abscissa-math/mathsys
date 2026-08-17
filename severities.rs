//^
//^ HEAD
//^

//> HEAD -> SYSTEMSTD
use systemstd::Severity;

//> HEAD -> STD
use std::panic::set_hook;


//^
//^ SEVERITIES
//^

//> SEVERITIES -> WARNING
pub enum Warning {} impl Severity for Warning {
    type Then = ();
    const COLOR: &'static str = "yellow";
    const SYMBOL: char = '#';
    fn done() -> Self::Then {}
}

//> SEVERITIES -> FAILURE
pub enum Failure {} impl Severity for Failure {
    type Then = !;
    const COLOR: &'static str = "red";
    const SYMBOL: char = '$';
    fn done() -> Self::Then {
        set_hook(Box::new(|_| ()));
        panic!();
    }
}

//> SEVERITIES -> COMPILER
pub enum Process {} impl Severity for Process {
    type Then = !;
    const COLOR: &'static str = "red";
    const SYMBOL: char = '&';
    fn done() -> Self::Then {
        set_hook(Box::new(|_| ()));
        panic!();
    }
}