use clap::Parser;
mod args;
use args::MyArgs;
use std::env;
use std::fs;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long, value_parser)]
    name: String,

    /// Number of times to greet
    #[clap(short, long, value_parser, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    // let filename = &args[2];
    println!("{:?}", query)
    // match &cli.command {}
    // for _ in 0..args.count {
    //     println!("Hello {}!", args.name)
    // }
}
