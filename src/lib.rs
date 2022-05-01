use dotenv::dotenv;
use reqwest::header::{HeaderMap, AUTHORIZATION};
use std::env;
use serde::Deserialize;
use chrono::{Utc, Duration, Date};

//Structs are used in deserialization
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
    //Returns article at a particular index
    pub fn article_at(&self, idx: usize) -> &Article {
        return &self.articles[idx];
    }
    //Returns url 
    pub fn url_at(&self, idx: usize) -> String {
        return self.article_at(idx).url.clone();
    }
}

//Helper function to retrieve news articles using an API
#[tokio::main]
pub async fn retrieve_news_articles() -> Result<Data, Box<dyn std::error::Error>> {

    //Setting up url to get news article from the week
    let now: Date<Utc> = Utc::now().date();
    let end: Date<Utc> = now - Duration::days(1);
    let mut url: String = "https://newsapi.org/v2/everything?q=keyword&sortBy=popularity&language=en".to_owned();
    url += "&from=";
    url += &now.format("%Y-%m-%d").to_string();
    url += "&to=";
    url += &end.format("%Y-%m-%d").to_string();
    url += "&en";
    println!("URL:{:?}", url); 

    dotenv().ok();
    //Retrieve API key from .env file
    let api_key = env::var("NEWS_API_KEY")?;
    
    //Attaching API key to Authorization HTTP header
    let mut header = HeaderMap::new();
    header.insert(AUTHORIZATION, api_key.parse().unwrap());

    //Sending Request using reqwest
    let client = reqwest::Client::new();
    
    let response = client
        .get(url)
        .headers(header)
        .send()
        .await?;

    // println!("{:#?}", response);

    //Deserialize response into Data objects
    let response_json = response.json::<Data>().await?;

    // println!("Response JSON: {:#?}", response_json);

    Ok(response_json)
}
