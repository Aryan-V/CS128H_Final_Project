pub mod lib;
use fltk::enums::CallbackTrigger;
use lib::Data;
use rust_bert::pipelines::zero_shot_classification::ZeroShotClassificationModel;
use fltk::{app, prelude::*, window::Window};
use fltk::*;



fn main() { 
    let app = app::App::default();

    let mut wind = Window::new(200, 200, 150, 250, "Hello");
    
    let f = frame::Frame::new(0,0,0,50,"Enter article to perform sentiment analysis on");


    let input = input::MultilineInput(0,0,50,50, "");
    
    // wind.end();

    // wind.show();
    
    app.run().unwrap();

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
    let candidate_labels = &["angry", "anticipation", "happy", "trust", "fear", "surprise", "sad", "disgust"];

    let sequence_classification_model = ZeroShotClassificationModel::new(Default::default()).unwrap();
    let output = sequence_classification_model.predict(
        input.as_slice(),
        candidate_labels,
        None,
        128,
    );
    let output1;
    for i in output {
        let frame1 = frame::Frame::new(0,0,0,50, i.text);
        output1 = input::IntInput(0,0,0,50, i.score);
    
    }
    wind.end();
    wind.show();
    
    input.set_trigger(CallbackTrigger::Changed);
    output1.set_trigger(CallbackTrigger::Changed);

    let (s, r) = app::channel::<bool>();

    \

    while app.wait().unwrap() {
        match r.recv() {
            Some(msg) => {

            }
            None => (),
        }

    }





    
    app.run().unwrap();
    print!("{:?}",output);
}
