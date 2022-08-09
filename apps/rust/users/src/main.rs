// mod mongo-connect;
// use crate::libs::rust::mongo-connect;

const ECHO_SERVER_ADDRESS: &str = "localhost:1234";

fn main() {
    println!("Hello, world!");
    let _grade = Some("A+");
    let grades = vec!["B-", "C+", "D"];
    // grades.extend(grade);

    // let gradess: Vec<Option<&str>> = grades.iter().collect();
    // println!("my item {}", gradess[0]);
    println!("{grades:?}");
}
