// mod libs::rust

fn main() {
    println!("Hello, world!");
    let _grade = Some("A+");
    let grades = vec!["B-", "C+", "D"];
    // grades.extend(grade);

    // let gradess: Vec<Option<&str>> = grades.iter().collect();
    // println!("my item {}", gradess[0]);
    println!("{grades:?}");
}
