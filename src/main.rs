use std::env;
use std::process;
use minigrep::Config;
fn main() {
    let args = env::args().collect::<Vec<String>>();
    // SAVING ARGUMENTS
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    //READ FILE
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
