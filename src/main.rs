use std::env;
use url_pest_parser::*;

fn print_help() {
    println!("URL Parser CLI - Usage:");
    println!("  cargo run <URL>         Parses a URL and displays its components.");
    println!("  cargo run -- --help        Displays help information.");
    println!("  cargo run -- --credits     Shows project credits.");
}

fn print_credits() {
    println!("URL Parser by Negrub Andrii");
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return Ok(());
    }

    match args[1].as_str() {
        "--help" => print_help(),
        "--credits" => print_credits(),
        url => match ParsedURL::from_url(url) {
            Ok(parsed) => println!("{:#?}", parsed),
            Err(e) => eprintln!("Error: {}", e),
        },
    }

    Ok(())
}
