use clap::Parser;
mod args;
// use args::MyArgs;
use reqwest::get;
use serde::Deserialize;
// use std::env;
// use std::fs;
use ureq::Error;

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

trait Battle {
    fn deal_damage(&self) -> i32;
}

trait Api<T> {
    fn get(&self) -> Result<T, Error>;
}

#[derive(Debug, Deserialize)]
pub struct User {
    id: String,
    firstname: String,
    lastname: String,
}

#[derive(Debug, Deserialize)]
pub struct Project {
    id: String,
    name: String,
    friends: Vec<User>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let args: Vec<String> = env::args().collect();
    // let query = &args[1];
    // let filename = &args[2];
    let url = "http://0.0.0.0:8080/api/projects";
    // let url1 = "https://httpbin.org/ip";
    let resp = get(url).await?.json::<Vec<Project>>().await?;
    println!("{}", resp[0].name);
    println!("{}", resp[0].id);
    println!("{:#?}", resp);
    Ok(())
    // match &cli.command {}
    // for _ in 0..args.count {
    //     println!("Hello {}!", args.name)
    // }
}
