extern crate minigrep;
use minigrep::Config;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let config = Config::new(&args).unwrap_or_else(|err| {
        std::process::exit(1);
    });


    if let Err(e) = minigrep::run(config) {
        std::process::exit(1);
    };
}
