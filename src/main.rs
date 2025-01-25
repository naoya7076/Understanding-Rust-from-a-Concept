const N_MAX: usize = 100000000;
const N_THREADS: usize = 4;

const N_ELEM_PER_THREAD: usize = N_MAX / N_THREADS;
const RESIDUAL: usize = N_MAX % N_THREADS;

fn main() -> std::thread::Result<()> {
    if RESIDUAL != 0 {
        panic!("N_MAX must be divisible by N_THREADS");
    }

    let mut thread = Vec::new();
    let v = std::sync::Arc::new((1..=N_MAX).collect::<Vec<usize>>());

    for i in 0..N_THREADS {
        let start = i * N_ELEM_PER_THREAD;
        let end = start + N_ELEM_PER_THREAD;
        let vv = std::sync::Arc::clone(&v);
        let th = std::thread::spawn(move || vv[start..end].iter().sum::<usize>());
        thread.push(th);
    }

    let ans: usize = thread.into_iter().map(|r| r.join().unwrap()).sum::<usize>();
    println!("ans: {}", ans);
    assert_eq!(ans, N_MAX * (N_MAX + 1) / 2);
    Ok(())
}
