use std::env;
use std::process;
use txt_to_pdf::Config;
fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem encountered parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = txt_to_pdf::generate_pdf(config) {
        println!("App error {e}");
        process::exit(1)
    }
}
