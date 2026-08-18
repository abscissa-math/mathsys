//^
//^ HEAD
//^

//> HEAD -> FEATURES
#![feature(const_trait_impl)]

//> HEAD -> CRITERION
use criterion::{
    Criterion,
    Throughput,
    criterion_main,
    criterion_group
};

//> HEAD -> MATHSYS
use mathsys::{
    Interpreter,
    Runtime,
    Error
};

//> HEAD -> CORE
use core::hint::{
    black_box,
    unreachable_unchecked
};


//^
//^ BENCHES
//^

//> BENCHES -> SETUP
criterion_group!(tokenizer, benches);
criterion_main!(tokenizer);

//> BENCHES -> RUN
fn benches(criterion: &mut Criterion) -> () {
    let mut group = criterion.benchmark_group("tokenizer");
    group.throughput(Throughput::Bytes(include_bytes!("../data/root.msm").len() as u64));
    struct Handler; impl<'valid> Runtime<'valid> for Handler {
        fn resolve(&'valid self, module: &'valid str) -> &'valid [u8] {return match module {
            "data/root.msm" => include_bytes!("../data/root.msm"),
            _ => unsafe {unreachable_unchecked()}
        }}
        fn error(_error: Error<'valid>) -> ! {panic!()}
    }
    let interpreter = Interpreter::from(Handler);
    group.bench_function("full", |bencher| bencher.iter(|| {
        let result = interpreter.latex("data/root.msm");
        black_box(result);
    }));
}