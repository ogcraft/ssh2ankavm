use std::env;
use ssh2ankavm::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);
    println!("config: {:?}", config);

}


