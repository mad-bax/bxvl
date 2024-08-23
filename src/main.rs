use std::process::exit;

use v3::VERSION;

fn print_help() {
    println!("HELP!");
    exit(0);
}

fn print_version() {
    println!("v{VERSION}");
    exit(0);
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    for arg in args {
        if arg == "-h" || arg == "--help" {
            print_help();
        } else if arg == "-v" || arg == "--version" {
            print_version();
        }
    }
}