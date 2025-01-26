use std::sync::{Arc, Mutex};
use std::thread::spawn;

fn main() {
    let data = Arc::new(Mutex::new(Vec::new()));

    let added = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let mut thread = Vec::new();
    for a in added {
        let data = Arc::clone(&data);
        let th = spawn(move || {
            let mut data = data.lock().unwrap();
            data.push(a);
        });
        thread.push(th);
    }
    thread.into_iter().for_each(|th| {
        let _ = th.join();
    });

    let x = data.lock().unwrap();
    println!("{:?}", x);
}
