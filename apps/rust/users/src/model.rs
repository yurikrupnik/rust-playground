use crate::{bson, doc, handle_create, post, Error};
use actix_web::{web, HttpResponse, Responder};
use futures::{StreamExt, TryStream, TryStreamExt};
use mongodb::bson::oid::ObjectId;
use mongodb::results::{DeleteResult, InsertOneResult};
use mongodb::{Client, Collection};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::arch::asm;
use std::fmt::format;

use fake::{Dummy, Fake, Faker};
use rand::rngs::StdRng;
use rand::SeedableRng;

#[derive(Debug, Dummy)]
pub struct Foo {
    #[dummy(faker = "1000..2000")]
    order_id: usize,
    customer: String,
    paid: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
}

// impl Default for User {
//     fn default() -> Self {
//         User {
//             id: None,
//             first_name: String::from("asdasd"),
//             last_name: String::from("dasd"),
//             username: String::from("asd"),
//             email: String::from("sdasd"),
//         }
//     }
// }

use async_trait::async_trait;
#[async_trait]
pub trait Api<T> {
    // fn create() -> Self,
    async fn get_all(&self, db: MongoRepo<T>, body: web::Json<T>) -> HttpResponse;
    // fn create(&self, d: T) -> Self;
}

pub struct ApiMethods {
    url: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Todo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
}

pub async fn get_todos() -> impl Responder {
    let aris = ApiMethods::new("bnir".to_string());
    let boo = aris.has_slash();

    HttpResponse::Ok().json(format!(
        "dam bool {} and url {} and second {}",
        boo,
        aris.url,
        aris.get_url()
    ))
}

pub struct MongoRepo<T> {
    col: Collection<T>,
}

impl<T> MongoRepo<T>
where
    T: Serialize + DeserializeOwned + Sync + Send + Unpin,
{
    pub async fn init(db_name: &str, col_name: &str) -> Self {
        let uri =
            std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
        let client = Client::with_uri_str(uri).await.expect("failed to connect");
        let col = client.database(db_name).collection(col_name);
        Self { col }
    }
    pub async fn create(&self, new_user: T) -> Result<InsertOneResult, Error> {
        let item = self
            .col
            .insert_one(new_user, None)
            .await
            .expect("Error creating item");
        Ok(item)
    }
    pub async fn delete(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = bson::oid::ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .delete_one(filter, None)
            .await
            .expect("Error deleting item");

        Ok(user_detail)
    }
    pub async fn list(&self) -> Result<Vec<T>, Error> {
        let mut cursor = self
            .col
            .find(None, None)
            .await
            .expect("Error getting list of users");
        let mut users: Vec<T> = Vec::new();
        while let Some(user) = cursor
            .try_next()
            .await
            .expect("Error mapping through cursor")
        {
            users.push(user)
        }
        Ok(users)
    }
}

#[async_trait]
impl<T> Api<T> for MongoRepo<T>
where
    T: Serialize + DeserializeOwned + Sync + Send + Unpin,
{
    async fn get_all(&self, db: MongoRepo<T>, body: web::Json<T>) -> HttpResponse {
        // db.col.
        // db.col.list;
        // Self::get_all()
        //   self::MongoRepo::list()
        let result = self::MongoRepo::list(self).await;
        match result {
            Ok(res) => HttpResponse::Ok().json(res),
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        }
        // db.col.de
        // db.col.
        // let results = db.list().await;
        // match results {
        //     Ok(res) => HttpResponse::Ok().json(res),
        //     Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        // }
        // HttpResponse::Ok().json("ad")
    }
}
impl ApiMethods {
    fn new(url: String) -> Self {
        Self {
            url: format!("/{}", url),
        }
    }
    fn get_url(&self) -> String {
        format!("shit happens {}", self.url)
    }

    fn has_slash(&self) -> bool {
        self.url.contains("aris")
    }
}

fn add(left: i32, right: i32) -> i32 {
    left + right
}
fn sub(left: i32, right: i32) -> i32 {
    left - right
}
fn select(name: &str) -> fn(i32, i32) -> i32 {
    match name {
        "add" => add,
        "sub" => sub,
        _ => unimplemented!(),
    }
}
fn sd() -> i32 {
    let fun = select("add");
    fun(1, 2)
}

pub async fn get_all_items(db: web::Data<MongoRepo<User>>) -> HttpResponse {
    // db
    let results = db.list().await;
    match results {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// fn adds<T>(db: web::Data<MongoRepo<T>>, body: web::Json<T>) -> HttpResponse {
//     let user = User {
//         id: None,
//         ..body.into_inner()
//     };
//     let result = db.create(user).await;
//     handle_create(result)
// }

// async fn ds<T>(db: web::Data<MongoRepo<T>>) -> HttpResponse {
//     // "SFdf".to_string()
//     let user = User {
//         id: None,
//         // ..body.into_inner()
//         first_name: "".to_string(),
//         last_name: "".to_string(),
//         username: "".to_string(),
//         email: "".to_string(),
//     };
//     let result = db.create(user).await;
//     handle_create(result)
// }
//
// fn das<T>() -> fn(ds: web::Data<MongoRepo<T>>) -> HttpResponse {
//     return ds;
// }
//
// fn dsdsd() {
//     let sd = das::<User>();
// }
