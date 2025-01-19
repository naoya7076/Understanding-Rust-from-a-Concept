use crate::RecursiveEnum::{Null, Val};

#[derive(Debug)]
enum RecursiveEnum {
    Val(Box<RecursiveEnum>),
    Null,
}

fn main() {
    let val = Val(Box::new(Val(Box::new(Val(Box::new(Null))))));
    println!("{:?}", val);
}
