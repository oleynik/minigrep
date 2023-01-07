use std::env;
use std::process;
use minigrep::Args;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = Args::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Query: {}", args.query);
    println!("File Name: {}", args.file_name);
    println!();

    if let Err(e) = minigrep::run(args) {
        eprintln!("Application error: {}", e);
        process::exit(2);
    }
}
