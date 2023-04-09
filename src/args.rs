use std::env;

pub struct Args {
    pub input_filename: String,
}

pub fn parse_args() -> Result<Args, &'static str> {
    let args: Vec<String> = env::args().collect();
    let mut input_filename: Option<&String> = None;
    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "-h" => {
                println!("USAGE: makurust filename.md");
                return Err("help");
            }
            "-v" => {
                println!("Makurust V 0.2beta");
                return Err("version");
            }
            _ => input_filename = Some(arg),
        }
    }
    let input_filename = match input_filename {
        Some(filename) => filename.to_string(),
        None => {
            println!("No input file provided!");
            return Err("input file not provided");
        }
    };
    Ok(Args { input_filename })
}
