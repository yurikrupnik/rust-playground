// mod libs::rust

fn main() {
    println!("Hello, world!");
    let grade = Some("A+");
    let grades = vec!["B-", "C+", "D"];
    // grades.extend(grade);

    let gradess: Vec<Option<&str>> = grades.iter().flatten().collect();
    // println!("my item {}", gradess[0]);
    println!("{gradess:?}");
}
