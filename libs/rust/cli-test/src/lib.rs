use std::error::Error;
// use std::fs;
//
// pub struct Config {
//     pub query: String,
//     pub filename: String,
// }
//
// impl Config {
//     pub fn new(args: &[String]) -> Result<Config, &str> {
//         if args.len() < 3 {
//             return Err("not enouegh arguments");
//         }
//         let query = args[1].clone();
//         let filename = args[2].clone();
//         Ok(Config { query, filename })
//     }
// }

pub fn run() -> Result<(), Box<dyn Error>> {
    // let contents = fs::read_to_string(config.filename);
    // println!("with text:\n{}", contents);
    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//       let query = "duck";
//       let contents = "\
//   Rust:
// safe, fast, productive.
// Pick three
//       ";
//         // assert_eq!(cli_test(), "cli_test".to_string());
//     }
// }
