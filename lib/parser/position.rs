//^
//^ POSITION
//^

//> POSITION -> STRUCT
#[derive(Clone, Default)]
pub struct Position {
    pub index: usize,
    pub line: usize,
    pub column: usize
}