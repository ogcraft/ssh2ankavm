use ssh2ankavm::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);
    println!("config: {:?}", config);
}
