//! Example code for using MongoDB with Actix.

extern crate core;

mod model;
mod product;
#[cfg(test)]
mod test;

use actix_web::{
    delete, get, guard, http, middleware::Logger, post, web, App, HttpRequest, HttpResponse,
    HttpServer, Responder, Result, Route,
};
use futures::TryFutureExt;
use model::{get_todos, MongoRepo, User};
use product::Product;
use std::borrow::Borrow;

use mongodb::{bson, bson::doc, options::IndexOptions, Client, Collection, IndexModel};
use mongodb::{bson::extjson::de::Error, results::InsertOneResult};

use futures::future::err;
use serde::{Deserialize, Serialize};
use std::env;
use std::env::VarError;
use std::fs::File;
use std::io::{ErrorKind, Read};
// use tokio::time::error::Error;

const DB_NAME: &str = "rustApp";
const COLL_NAME: &str = "users";

#[derive(Serialize)]
struct MyObj {
    name: String,
}

#[get("/file")]
async fn read_file() -> Result<impl Responder> {
    // let f = File::open("hello.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => HttpResponse::InternalServerError().body("ads"),
    //     // Err(error) => panic!("problem reading the file {:?}", error),
    // };
    // HttpResponse::Ok().body("sdas")
    Ok(web::Json("s"))
}

#[get("/a/{name}")]
async fn index(name: web::Path<String>) -> Result<impl Responder> {
    let obj = MyObj {
        name: name.to_string(),
    };
    Ok(web::Json(obj))
}

/// Gets the user with the supplied username.
#[get("/users/{id}")]
async fn get_user(client: web::Data<Client>, path: web::Path<String>) -> HttpResponse {
    // let username = username.into_inner();
    let id = path.into_inner();
    if id.is_empty() || id.len() != 24 {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    let collection: Collection<User> = client.database(DB_NAME).collection(COLL_NAME);
    let obj_id = bson::oid::ObjectId::parse_str(id).unwrap();
    let filter = doc! {"_id": obj_id};
    match collection.find_one(filter, None).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body(format!("No user found with username")),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// Done
#[post("/users")]
pub async fn add_user(db: web::Data<MongoRepo<User>>, body: web::Json<User>) -> HttpResponse {
    let user = User {
        id: None,
        ..body.into_inner()
    };
    let result = db.create(user).await;
    handle_create(result)
}

fn handle_create(result: Result<InsertOneResult, Error>) -> HttpResponse {
    match result {
        Ok(res) => HttpResponse::Ok().json(res.inserted_id),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// Done
#[post("/products")]
pub async fn add_product(
    db: web::Data<MongoRepo<Product>>,
    body: web::Json<Product>,
) -> HttpResponse {
    let product = Product {
        id: None,
        ..body.into_inner()
    };
    let result = db.create(product).await;
    handle_create(result)
}

// Done!!!
#[delete("/users/{id}")]
pub async fn delete_user(db: web::Data<MongoRepo<User>>, path: web::Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() || id.len() != 24 {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    let result = db.delete(&id).await;
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                HttpResponse::Ok().json("successfully deleted!")
            } else {
                HttpResponse::NotFound().json("User with specified ID not found!")
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// Done!!
#[get("/users")]
pub async fn get_all_users(db: web::Data<MongoRepo<User>>) -> HttpResponse {
    let results = db.list().await;
    match results {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let uri = "mongodb+srv://yurikrupnik:T4eXKj1RBI4VnszC@cluster0.rdmew.mongodb.net/?retryWrites=true&w=majority";

    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    let client = Client::with_uri_str(uri).await.expect("failed to connect");
    // let client = Client::with_uri_str(uri).await.expect("failed to connect");
    // let db = client.database(DB_NAME).collection(COLL_NAME);
    // let client = db.collection(COLL_NAME);
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let repo = MongoRepo::<User>::init(DB_NAME, COLL_NAME).await;
    let repo1 = MongoRepo::<Product>::init(DB_NAME, "prouct").await;
    let db_data = web::Data::new(repo);
    let db_data1 = web::Data::new(repo1);
    // todo check how to make it work
    // let route = Route::new();
    // route.method(http::Method::GET).to(get_todos);
    HttpServer::new(move || {
        App::new()
            // .wrap(Logger::default())
            .app_data(db_data.clone())
            .app_data(db_data1.clone())
            .app_data(web::Data::new(client.clone()))
            .service(index)
            .service(read_file)
            .service(add_user)
            .service(add_product)
            .service(get_all_users)
            // .route("/users", web::get().to(repo.get_all))
            .service(delete_user)
            .service(get_user)
            // .route("/shit", route)
            .route(
                "/aris",
                web::get()
                    .method(http::Method::CONNECT)
                    .guard(guard::Header("content-type", "text/plain"))
                    .to(|req: HttpRequest| HttpResponse::Ok()),
            )
            .route(
                "/todos",
                web::get().to(get_todos), // .method(http::Method::POST)
                                          // .to(get_todos),
            )
            .service(
                web::resource("/path").route(
                    web::get()
                        .method(http::Method::CONNECT)
                        .guard(guard::Header("content-type", "text/plain"))
                        .to(|req: HttpRequest| HttpResponse::Ok()),
                ),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
