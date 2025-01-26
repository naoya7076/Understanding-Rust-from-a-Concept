async fn sum_func(n: usize) -> usize {
    let ans = (1..=n).sum::<usize>();
    println!("sum: {}", ans);
    ans
}

fn main() {
    let ls = tokio::task::LocalSet::new();
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    ls.block_on(&rt, async {
        sum_func(10000000).await;
        sum_func(20000000).await;
    });
}
