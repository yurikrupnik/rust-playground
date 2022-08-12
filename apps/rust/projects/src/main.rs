use colour::{dark_green, yellow};
extern crate dotenv;
use dotenv::dotenv;
use serde::Deserialize;
use std::error::Error;
// use thiserror::Error;
use ureq::get;

fn render_articles(articles: &Articles) {
    for i in &articles.articles {
        dark_green!("> {}\n", i.title);
        yellow!("> {}\n\n", i.url);
    }
}

#[derive(thiserror::Error, Debug)]
pub enum NewsApiError {
    #[error("Failed fetching articles")]
    RequestFailed(ureq::Error),
    #[error("Failed converting response to string")]
    FailedResponseToString(std::io::Error),
    #[error("Article parsing failed")]
    ArticleParseFailed(serde_json::Error),
}
#[derive(Deserialize, Debug)]
pub struct Articles {
    pub articles: Vec<Article>,
}

#[derive(Deserialize, Debug)]
pub struct Article {
    pub title: String,
    pub url: String,
}

pub fn get_articles(url: &str) -> Result<Articles, NewsApiError> {
    let response = get(url)
        .call()
        .map_err(NewsApiError::RequestFailed)?
        .into_string()
        .map_err(NewsApiError::FailedResponseToString)?;

    let articles: Articles =
        serde_json::from_str(&response).map_err(NewsApiError::ArticleParseFailed)?;
    Ok(articles)
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenv();
    let api_key = std::env::var("API_KEY")?;
    // println!("api_key {}", api_key);
    // for (key, value) in std::env::vars() {
    //     println!("{}: {}", key, value);
    // }
    let url = "https://newsapi.org/v2/top-headlines?country=us&apiKey=";
    let url = format!("{}{}", url, api_key);
    let articles = get_articles(&url)?;
    render_articles(&articles);
    Ok(())
}
