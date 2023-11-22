#[allow(dead_code)]
mod challenges;
fn main() -> () {
    let a = challenges::format_duration(1);
    println!("{:?}",a);
}
