use std::string;

use rust_bert::pipelines::zero_shot_classification::ZeroShotClassificationModel;
use futures::executor::block_on;
use lib::Article;
mod lib;

fn get_titles(to_copy: &Vec<Article>) -> Vec<String> {
    let mut article_titles = Vec::new();
    for article in to_copy {
        article_titles.push(article.title.clone());
    }
    article_titles
}

fn main() {
    let api_result = lib::retrieve_news_articles();

    let data = match api_result {
        Ok(data) => get_titles(&data.articles),
        _ => Vec::new()
    };

    // println!("here");
    for title in &data {
        print!("{:?}",title);
    }

    let sequence_classification_model = ZeroShotClassificationModel::new(Default::default()).unwrap();

    let input: Vec<&str> = data.iter().map(|s| s.as_ref()).collect();
    let candidate_labels = &["angry", "anticipation", "happy", "trust", "fear", "surprise", "sad", "disgust"];

    let output = sequence_classification_model.predict(
        input.as_slice(),
        candidate_labels,
        None,
        128,
    );
    print!("{:?}",output);
}
