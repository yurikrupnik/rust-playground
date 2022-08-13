use clap::Parser;

/// Simple program to greet a person
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct MyArgs {
    /// Name of the person to greet
    #[clap(short, long, value_parser)]
    pub name: String,

    /// Number of times to greet
    #[clap(short, long, value_parser, default_value_t = 1)]
    pub count: u8,
}

// #[derive(Debug, Subcommand)]
// pub enum EntityType {
//     /// Create, update, delete or show users
//     User(UserCoMmand),
//     /// Create, update, delete or show videos
//     Video(VideoCoMmand),
//     /// Create, update, delete or show views
//     View(ViewCoMmand),
// }
