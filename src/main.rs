fn do_it(f: fn(u64, u64) -> u64, a: u64, b: u64) {
    println!("Result: {}", f(a, b));
}

fn add(a: u64, b: u64) -> u64 {
    a + b
}

fn multiply(a: u64, b: u64) -> u64 {
    a * b
}

fn main() {
    do_it(add, 1, 2);
    do_it(multiply, 3, 4);
}
