use std::io;

fn main() {
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("An Unknown error occured");
    let inp:u16 = match inp.trim().parse() {
        Ok(inp) => inp,
        Err(_) => return
    };
    println!("Fibonacci at position {}: {}", inp, nth_fib(inp));
}

fn nth_fib(n:u16)->u64 {
    let mut prev = 0;
    let mut fib = 1;
    let mut tmp;
    for _ in 0..n {
        tmp = fib;
        fib = fib + prev;
        prev = tmp;
    };
    return fib;
}
