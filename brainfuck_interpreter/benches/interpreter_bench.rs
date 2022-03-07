use brainfuck_interpreter::interpret;
use criterion::{criterion_group, criterion_main, Criterion};

fn fibonacci() -> String {
    let source = "+++++++++++
    >+>>>>++++++++++++++++++++++++++++++++++++++++++++
    >++++++++++++++++++++++++++++++++<<<<<<[>[>>>>>>+>
    +<<<<<<<-]>>>>>>>[<<<<<<<+>>>>>>>-]<[>++++++++++[-
    <-[>>+>+<<<-]>>>[<<<+>>>-]+<[>[-]<[-]]>[<<[>>>+<<<
    -]>>[-]]<<]>>>[>>+>+<<<-]>>>[<<<+>>>-]+<[>[-]<[-]]
    >[<<+>>[-]]<<<<<<<]>>>>>[+++++++++++++++++++++++++
    +++++++++++++++++++++++.[-]]++++++++++<[->-<]>++++
    ++++++++++++++++++++++++++++++++++++++++++++.[-]<<
    <<<<<<<<<<[>>>+>+<<<<-]>>>>[<<<<+>>>>-]<-[>>.>.<<<
    [-]]<<[>>+>+<<<-]>>>[<<<+>>>-]<<[<+>-]>[<+>-]<<<-]";
    let input = "".as_bytes();

    let actual = interpret(source, Box::new(input));
    actual.expect("Program should work")
}

fn bench_fibonacci_interpreter(c: &mut Criterion) {
    c.bench_function("fib", |b| b.iter(|| fibonacci()));
}

criterion_group!(benches, bench_fibonacci_interpreter);
criterion_main!(benches);
