extern crate minigrep;
use minigrep::Config;

fn main() {
    let config = Config::new(std::env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });


    if let Err(err) = minigrep::run(config) {
        eprintln!("Problem running minigrep: {}", err);
        std::process::exit(1);
    };
}
