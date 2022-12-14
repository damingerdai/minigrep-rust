use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args = env::args();

    let config = Config::new(args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    eprintln!("seatching for {}", config.query);
    eprintln!("In file {}", config.filename);


    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}



