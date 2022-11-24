use crate::bson;
use fake::{Dummy, Fake};
// use crate::bson::Bson::ObjectId;
use fake::Faker;
use mongodb::bson::oid::ObjectId;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Dummy, Deserialize, Serialize)]
pub struct Foo {
    #[dummy(faker = "1000..2000")]
    order_id: usize,
    customer: String,
    paid: bool,
    shit: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Product {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
}

impl Default for Product {
    fn default() -> Self {
        let f: Foo = Faker.fake();
        println!("{:?}", f);
        Self {
            id: None,
            name: String::from("sdasd"),
        }
    }
}

impl Product {
    fn new(id: bool) -> Self {
        // Self::default()
        Self {
            id: if id { ObjectId::new().into() } else { None },
            name: String::from("sdasd"),
        }
    }
}

fn dsa() {
    Product::new(false);
}
