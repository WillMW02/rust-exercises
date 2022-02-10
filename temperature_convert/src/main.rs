use std::io;

fn main() {
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("An Unknown error occured");
    let inp:f32 = match inp.trim().parse() {
        Ok(inp) => inp,
        Err(_) => return
    };
    println!("C->F: {}, F->C: {}", c2f(inp), f2c(inp));
}

fn f2c(f:f32)->f32 {
    return (f - 32.) * (5. / 9.);
}

fn c2f(c:f32)->f32 {
    return (c * (9. / 5.)) + 32.;
}