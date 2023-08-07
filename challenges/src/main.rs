mod challenges;
fn main() -> () {
    let a = challenges::meeting("Fred:Corwill;Wilfred:Corwill;Barney:Tornbull;Betty:Tornbull;Bjon:Tornbull;Raphael:Corwill;Alfred:Corwill");
    println!("{a:?}");
}