async fn sum_func(n: usize) -> usize {
    let ans = (1..=n).sum::<usize>();
    println!("sum: {}", ans);
    ans
}
#[tokio::main]
async fn main() {
    sum_func(10000000).await;
    sum_func(20000000).await;
}
