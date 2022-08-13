// mod mongo-connect;
// use crate::libs::rust::mongo-connect;
// use log::{info, warn};

// use cli_test::cli_test;
use mongo_connect::auth_utils::models::Credentials;
use mongo_connect::authenticate;
// use super::

const ECHO_SERVER_ADDRESS: &str = "localhost:1234";
fn main() {
    let yuri = Credentials {
        username: "aris".to_string(),
        password: "123456".to_string(),
    };
    authenticate(yuri);
    println!("Hello, world!");
    println!("Running at url: {}", ECHO_SERVER_ADDRESS);
    let _grade = Some("A+");
    let grades = vec!["B+", "C+", "D"];
    // grades.extend(grade);

    // let gradess: Vec<Option<&str>> = grades.iter().collect();
    // println!("my item {}", gradess[0]);
    println!("{grades:?}");
}
