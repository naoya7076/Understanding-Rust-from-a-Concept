use std::{sync::mpsc::channel, thread::spawn};

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let (tx, rx) = channel::<i32>();

    let data_len = data.len();

    for d in data {
        let tx = tx.clone();
        spawn(move || tx.send(d));
    }

    for _ in 0..data_len {
        println!("{}", rx.recv().unwrap());
    }
}
