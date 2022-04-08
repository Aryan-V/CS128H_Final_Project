use dotenv::dotenv;
use reqwest::header::{HeaderMap, AUTHORIZATION};
use std::env;
use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct Source {
    id: Option<String>,
    name: String,
}
#[derive(Deserialize, Debug, Default)]
pub struct Article {
    source: Source,
    author: Option<String>,
    title: String,
    description: String,
    url: String, 
    #[serde(rename="urlToImage")]
    url_to_image: Option<String>,
    #[serde(rename="publishedAt")]
    published_at: String,
    content: String
}
#[derive(Deserialize, Debug, Default)]
pub struct Data {
    status: String,
    #[serde(rename="totalResults")]
    total_results: i32,
    articles: Vec<Article>
}


impl Data {
    //Collects all the article titles into a vector
    pub fn get_headlines(&self) -> Vec<String> {
        let mut article_headlines = Vec::new();
        for article in &self.articles {
            article_headlines.push(article.title.clone());
        }
        article_headlines
    }
}

//Helper function to retrieve news articles using an API
#[tokio::main]
pub async fn retrieve_news_articles() -> Result<Data, Box<dyn std::error::Error>> {
    dotenv().ok();
    //Retrieve API key from .env file
    let api_key = env::var("NEWS_API_KEY")?;
    
    //Attaching API key to Authorization HTTP header
    let mut header = HeaderMap::new();
    header.insert(AUTHORIZATION, api_key.parse().unwrap());

    //Sending Request using reqwest
    let client = reqwest::Client::new();
    
    let response = client
        .get("https://newsapi.org/v2/everything?q=keyword")
        .headers(header)
        .send()
        .await?;

    // println!("{:#?}", response);

    //Deserialize response into Data objects
    let response_json = response.json::<Data>().await?;

    // println!("Response JSON: {:#?}", response_json);

    Ok(response_json)
}
