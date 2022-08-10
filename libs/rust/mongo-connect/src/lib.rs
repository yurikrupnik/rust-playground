fn mongo_connect() -> String {
    "mongo_connect".to_string()
}

mod database {
    pub enum Status {
        Connected,
        Interrupted,
    }
    pub fn connect_to_database() -> Status {
        return Status::Connected;
    }
    pub fn get_user() {}
}

pub mod auth_utils {
    pub fn login(creds: models::Credentials) {
        crate::database::get_user()
    }

    fn logout() {}

    pub mod models {
        pub struct Credentials {
            pub username: String,
            pub password: String,
        }
    }
}

use auth_utils::login;
use auth_utils::models::Credentials;
use database::{connect_to_database, Status};

pub fn authenticate(creds: Credentials) -> bool {
    if let Status::Connected = connect_to_database() {
        login(creds);
    }
    return true;
}
// fn options() {
//   dbg!(hello("brooks")),
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            authenticate(Credentials {
                password: "123456".to_string(),
                username: "yuri".to_string()
            }),
            true
        );
        assert_eq!(mongo_connect(), "mongo_connect".to_string());
        // let unwrapped_name =
    }
}
