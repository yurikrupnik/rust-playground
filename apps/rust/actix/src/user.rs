use std::sync::Mutex;

use futures::stream::TryStreamExt;
use mongodb::{
    bson::doc, bson::oid::ObjectId, bson::Document, options::IndexOptions, Client, Collection,
    IndexModel,
};
use once_cell::sync::OnceCell;

use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path, Query, ServiceConfig},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use thiserror::Error;

const DB_NAME: &str = "myApp";
const COLL_NAME: &str = "users";

#[derive(Error, Debug)]
pub enum Error {
    #[error("MongoDB Error")]
    ErrorMongo(#[from] mongodb::error::Error),
}

pub type AppResult<T> = Result<T, Error>;

// #[derive(Debug, Deserialize, Serialize)]
// struct User {
//   _id: Option<ObjectId>,
//   first_name: String,
//   last_name: String,
//   username: String,
//   email: String,
// }

use crate::todo::Todo;
use crate::{ErrorResponse, LogApiKey, RequireApiKey, TodoStore};

// "mongodb+srv://yurikrupnik:T4eXKj1RBI4VnszC@cluster0.rdmew.mongodb.net/?retryWrites=true&w=majority"

#[derive(Default)]
pub(super) struct UserStore {
    users: Mutex<Vec<User>>,
}

pub(super) fn configure(store: Data<UserStore>) -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config
            .app_data(store)
            .service(search_users)
            .service(get_users)
            .service(create_user);
        // .service(delete_todo)
        // .service(get_todo_by_id)
        // .service(update_todo);
    }
}

/// Users here.
#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub(super) struct User {
    /// Unique id for the todo item.
    #[schema(example = 1)]
    id: i32,
    /// Description of the taks to do.
    #[schema(example = "Yuri")]
    name: String,
    /// User email account
    #[schema(example = "example@exampl.com")]
    email: String,
}

static MONGODB_CLIENT: OnceCell<Client> = OnceCell::new();

#[inline]
pub fn get_mongodb_client() -> &'static Client {
    unsafe { MONGODB_CLIENT.get_unchecked() }
}

/// Search users Query
#[derive(Deserialize, Debug, IntoParams)]
pub(super) struct SearchUsers {
    /// Content that should be found from User's value field
    value: String,
}

/// Get list of todos.
///
/// List todos from in-memory todo store.
///
/// One could call the api endpoit with following curl.
/// ```text
/// curl localhost:8080/users
/// ```
#[utoipa::path(
responses(
(status = 200, description = "List current users items", body = [User])
)
)]
#[get("/users")]
pub(super) async fn get_users(user_store: Data<UserStore>) -> impl Responder {
    let users = user_store.users.lock().unwrap();

    HttpResponse::Ok().json(users.clone())
}

/// Create new Todo to shared in-memory storage.
///
/// Post a new `User` in request body as json to store it. Api will return
/// created `User` on success or `ErrorResponse::Conflict` if todo with same id already exists.
///
/// One could call the api with.
/// ```text
/// curl localhost:8080/user -d '{"id": 1, "value": "Buy movie ticket", "checked": false}'
/// ```
#[utoipa::path(
request_body = User,
responses(
(status = 201, description = "User created successfully", body = User),
)
)]
#[post("/user")]
pub(super) async fn create_user(user: Json<User>, user_store: Data<UserStore>) -> impl Responder {
    // let client = get_mongodb_client();
    // let coll_users = client.database(DB_NAME).collection::<Document>(COLL_NAME);
    // let new_user = req.parse_json::<User>().await.unwrap();

    let mut users = user_store.users.lock().unwrap();
    let user = &user.into_inner();

    users
        .iter()
        .find(|existing| existing.id == user.id)
        .map(|existing| {
            HttpResponse::Conflict().json(ErrorResponse::Conflict(format!("id = {}", existing.id)))
        })
        .unwrap_or_else(|| {
            users.push(user.clone());

            HttpResponse::Ok().json(user)
        })
}

/// Search Users with by value
///
/// Perform search from `User`s present in in-memory storage by matching Todo's value to
/// value provided as query paramter. Returns 200 and matching `User` items.
#[utoipa::path(
params(
SearchUsers
),
responses(
(status = 200, description = "Search Todos did not result error", body = [User]),
)
)]
#[get("/user/search")]
pub(super) async fn search_users(
    query: Query<SearchUsers>,
    user_store: Data<UserStore>,
) -> impl Responder {
    let users = user_store.users.lock().unwrap();

    HttpResponse::Ok().json(
        users
            .iter()
            // .filter(|user| {
            //     user.value
            //         .to_lowercase()
            //         .contains(&query.value.to_lowercase())
            // })
            .cloned()
            .collect::<Vec<_>>(),
    )
}
