#[allow(dead_code)]
mod challenges;
fn main() -> () {
    let a = challenges::sum_intervals(&[(-1_000_000_000, 1_000_000_000)]);
    println!("{:?}",a);
}
