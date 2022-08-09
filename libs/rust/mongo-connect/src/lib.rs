fn mongo_connect() -> String {
    "mongo_connect".to_string()
}

// fn options() {
//   dbg!(hello("brooks"))
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(mongo_connect(), "mongo_connect".to_string());
        // let unwrapped_name =
    }
}
