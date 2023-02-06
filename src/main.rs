use std::env;
use std::process;
use minigrep::Config;
fn main() {
    let args = env::args().collect::<Vec<String>>();
    // SAVING ARGUMENTS
    let config = Config::build(env::args() ).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    //READ FILE
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
