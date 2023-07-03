use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=3);
    
    loop {
        
        println!("Please input your guess");

        let mut guess = String::new();
        io::stdin()
        .read_line( &mut guess)
        .expect("Failed to read line");
        let guess = match guess.trim().parse::<u32>() {
            Ok(num)=> num,
            Err(_) => continue,
        }; //.expect("Please type a correct number");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {println!("You win");
                                break;
                                },
            Ordering::Greater => println!("Too big"),
        }
        println!("Your last guessed: {guess}");
    }
    
}
