//^
//^ HEAD
//^

//> HEAD -> MATHSYS
use mathsys::{
    Runtime,
    Error
};

//> HEAD -> SYSTEMSTD
use systemstd::{
    System,
    Handling,
    Read
};

//> HEAD -> ELSA
use elsa::FrozenMap;


//^
//^ HANDLER
//^

//> HANDLER -> STRUCT
#[derive(Default)]
pub struct Handler<'valid> {
    cache: FrozenMap<&'valid str, Vec<u8>>
}

//> HANDLER -> RUNTIME
impl<'valid> Runtime<'valid> for Handler<'valid> {
    fn resolve(&'valid self, module: &'valid str) -> &'valid [u8] {
        return match self.cache.get(module) {
            Some(cached) => cached,
            None => self.cache.insert(module, System::expect(
                System::expect(System::path(module).file::<Read>(
                    Handling::AssumeExists
                )).read_bytes()
            ))
        }
    }
    fn error(error: Error<'valid>) -> ! {System::error(error)}
}