use std::process;

mod cli;

fn main() {
    let matches = cli::build_cli().get_matches();
    if let Err(e) = cli::parse_config(&matches).and_then(|config| lop::run(&config)) {
        eprintln!("Error: {}", e);
        eprintln!("{}", matches.usage());
        process::exit(1);
    }
}
