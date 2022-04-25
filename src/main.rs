pub mod lib;
use fltk::enums::CallbackTrigger;
use lib::Data;
use rust_bert::pipelines::zero_shot_classification::ZeroShotClassificationModel;
use fltk::{app, prelude::*, window};
use fltk::*;
use std::collections::HashMap;
use std::hash::Hash;


fn create_app() -> fltk::app::App {
    let app = app::App::default();
    let mut win = window::Window::default().with_size(600, 400).with_label("Window");
    
    let mut frame = frame::Frame::new(0,0,0,50,"Enter article to perform sentiment analysis on");
    let input = input::MultilineInput::new(0,0,50,50, "");
    win.end();
    frame.show();
    win.show();
    app 
}

fn classify(newsarticles : Data) -> Vec<HashMap<String, String>> {
    let headlines = newsarticles.get_headlines();
    let mut sections: Vec<HashMap<String,String>> =  Vec::new();

    // for title in data.get_headlines() {
    //     print!("{:?}\n",title);
    // }

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
        let mut section: HashMap<String,String> = HashMap::new();
        // println!("{}", candidate_label);

        for i in 0..output.len() {
            if &(output[i].text)[..] == *candidate_label && output[i].score > 0.20 {
                section.insert(headlines[i].clone(), newsarticles.url_at(i));
                // println!("{}", headlines[i]);
                // println!("{}", data.url_at(i));
            }
        }
        sections.push(section);
    }

    sections

}

fn main() { 
    let response = lib::retrieve_news_articles();
    let data: Data = match response {
        Ok(data) => data,
        _ => panic!("Did not find data")
    };

    // let labels = ["angry", "happy", "silly", "fear", "surprise", "sad", "disgust", "suspense", "neutral"];
    // let sections = classify(data);

    // for section in 0..sections.len() {
    //     println!("{:?}", labels[section]);
    //     for article in &sections[section]{
    //         println!("{:?}: {:?}", article.0, article.1);
    //     }
    // }
    
    let app = create_app();
    app.run().unwrap();

    // let output1;
    // for i in output {
    //     let frame1 = frame::Frame::new(0,0,0,50, i.text.clone());
    //     output1 = input::IntInput(0,0,0,50, i.score);
    
    // } 

    // input.set_trigger(CallbackTrigger::Changed);
    // output1.set_trigger(CallbackTrigger::Changed);

    // let (s, r) = app::channel::<bool>();


    // while app.wait().unwrap() {
    //     match r.recv() {
    //         Some(msg) => {

    //         }
    //         None => (),
    //     }

    // }
    // app.run().unwrap();
}
