// mod mongo-connect;
// use crate::libs::rust::mongo-connect;
// use log::{info, warn};
//
// struct Yak {
//     name: String,
// }
//
// fn find_a_razor() {}
//
// pub fn shave_the_yak(yak: &mut Yak) {
//     // info!(target: "yak_events", "Commencing yak shaving for {:?}", yak);
//     info!(target: "yak_events", "Commencing yak shaving for {:?}", yak.name);
//
//     loop {
//         match find_a_razor() {
//             Ok(razor) => {
//                 info!("Razor located: {}", razor);
//                 yak.shave(razor);
//                 break;
//             }
//             Err(err) => {
//                 warn!("Unable to locate a razor: {}, retrying", err);
//             }
//         }
//     }
// }
const ECHO_SERVER_ADDRESS: &str = "localhost:1234";

fn main() {
    println!("Hello, world!");
    println!("Running at url: {}", ECHO_SERVER_ADDRESS);
    let _grade = Some("A+");
    let grades = vec!["B-", "C+", "D"];
    // grades.extend(grade);

    // let gradess: Vec<Option<&str>> = grades.iter().collect();
    // println!("my item {}", gradess[0]);
    println!("{grades:?}");
}
