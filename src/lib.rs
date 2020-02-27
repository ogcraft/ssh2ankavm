use std::error::Error;


#[derive(Debug)]
pub struct Config  {
    pub vm_name: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        println!("args: {:?}", args);
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let vm_name = args[1].clone();

        Ok(Config { vm_name: vm_name})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("config: {:?}", config);
    Ok(()) 
}
