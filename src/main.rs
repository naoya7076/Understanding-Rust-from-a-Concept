use thiserror::Error;

#[derive(Error, Debug)]
enum DivError {
    #[error("{0} divided by zero")]
    DivideByZero(i32),
    #[error("Both {0} and {1} are negative")]
    BothNegative(i32, i32),
}

fn mydiv(a: i32, b: i32) -> Result<i32, DivError> {
    if b == 0 {
        return Err(DivError::DivideByZero(a));
    }
    if a < 0 && b < 0 {
        return Err(DivError::BothNegative(a, b));
    }
    Ok(a / b)
}

fn print_mydiv(a: i32, b: i32) {
    match mydiv(a, b) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => eprintln!("{}", e),
    }
}

fn main() {
    print_mydiv(10, 2);
    print_mydiv(10, 0);
    print_mydiv(-10, -2);
}
