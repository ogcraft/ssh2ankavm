use ssh2ankavm::*;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap();
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
