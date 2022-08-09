pub fn mongo_connect() -> String {
    "mongo_connect".into()
}

fn options() {
  dbg!(hello(Some("brooks")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(mongo_connect(), "mongo_connect".to_string());
        let unwrapped_name =
    }
}
