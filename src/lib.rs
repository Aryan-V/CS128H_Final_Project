use dotenv::dotenv;
use reqwest::header::{HeaderMap, AUTHORIZATION};
use std::env;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Source {
    id: Option<String>,
    name: String,
}
#[derive(Deserialize, Debug)]
pub struct Article {
    source: Source,
    author: Option<String>,
    pub title: String,
    description: String,
    url: String, 
    urlToImage: Option<String>,
    publishedAt: String,
    content: String
}
#[derive(Deserialize, Debug)]
pub struct Data {
    status: String,
    totalResults: i32,
    pub articles: Vec<Article>
}

#[tokio::main]
pub async fn retrieve_news_articles() -> Result<Data, Box<dyn std::error::Error>> {
    dotenv().ok();

    let api_key = env::var("NEWS_API_KEY")?;
    
    let mut header = HeaderMap::new();
    header.insert(AUTHORIZATION, api_key.parse().unwrap());

    let client = reqwest::Client::new();
    let response = client
        .get("https://newsapi.org/v2/everything?q=keyword")
        .headers(header)
        .send()
        .await?;

    println!("{:#?}", response);

    //Deserialize reponse into Data objects
    let response_json = response.json::<Data>().await?;

    println!("Response JSON: {:#?}", response_json);

    Ok(response_json)
}