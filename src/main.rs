pub mod lib;
use lib::Data;
use rust_bert::pipelines::zero_shot_classification::ZeroShotClassificationModel;

fn main() {
    let response = lib::retrieve_news_articles();

    let data: Data = match response {
        Ok(data) => data,
        _ => panic!("Did not find data")
    };

    for title in data.get_headlines() {
        print!("{:?}\n",title);
    }

    let headlines = data.get_headlines();
    let input: Vec<&str> = headlines.iter().map(|s| s.as_ref()).collect();
    let candidate_labels = &["angry", "happy", "silly", "fear", "surprise", "sad", "disgust", "suspense", "neutral"];

    let sequence_classification_model = ZeroShotClassificationModel::new(Default::default()).unwrap();
    let output = sequence_classification_model.predict(
        input.as_slice(),
        candidate_labels,
        None,
        128,
    );

    // print!("{:?}",output);

    for candidate_label in candidate_labels {
        println!("{}", candidate_label);
        for i in 0..output.len() {
            if &(output[i].text)[..] == *candidate_label && output[i].score > 0.20 {
                println!("{}", headlines[i]);
                println!("{}", data.url_at(i));
            }
        }
    }

}
