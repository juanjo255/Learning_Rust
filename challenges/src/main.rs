mod challenges;
fn main() -> () {
    let a = challenges::revrot("123456779", 8);
    println!("{a}");
}