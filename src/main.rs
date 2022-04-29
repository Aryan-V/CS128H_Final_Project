pub mod lib;
use lib::Data;
use rust_bert::pipelines::zero_shot_classification::ZeroShotClassificationModel;
use fltk::{app, prelude::*, window};
use fltk::*;
use fltk::enums::CallbackTrigger;

pub struct HeadlineURLPair {
    headline: String,
    url: String,
}

fn create_app() -> fltk::app::App {
    let app = app::App::default().with_scheme(app::Scheme::Plastic);
    let mut win = window::Window::default().with_size(1400, 1000).center_screen();
    
    let mut frame = frame::Frame::new(80,0,50,50,"Recommended Reads Today");
    frame.set_label_font(enums::Font::HelveticaBold);

    let response = lib::retrieve_news_articles();
    let data: Data = match response {
        Ok(data) => data,
        _ => panic!("Did not find data")
    };

    let classified_vec: Vec<Vec<HeadlineURLPair>> = classify(data);

    let frame_dramatic = frame::Frame::new(0,30,50,50,"Dramatic Stories").with_align(enums::Align::Right);
    let frame_dramatic_link1 = frame::Frame::new(10,50,50,50,"link").with_align(enums::Align::Right);
    frame_dramatic_link1.with_label(classified_vec[0][0].headline.as_str());
    let frame_dramatic_link2 = frame::Frame::new(10,70,50,50, "link").with_align(enums::Align::Right);
    frame_dramatic_link2.with_label(classified_vec[0][1].headline.as_str());
    let frame_dramatic_link3 = frame::Frame::new(10,90,50,50,"link").with_align(enums::Align::Right);
    frame_dramatic_link3.with_label(classified_vec[0][2].headline.as_str());
    let frame_uplifting = frame::Frame::new(0,120,50,50,"Uplifting Stories").with_align(enums::Align::Right);
    let frame_uplifting_link1 = frame::Frame::new(10,140,50,50,"link").with_align(enums::Align::Right);
    frame_uplifting_link1.with_label(classified_vec[1][0].headline.as_str());
    let frame_uplifting_link2 = frame::Frame::new(10,160,50,50,"link").with_align(enums::Align::Right);
    frame_uplifting_link2.with_label(classified_vec[1][1].headline.as_str());
    let frame_uplifting_link3 = frame::Frame::new(10,180,50,50,"link").with_align(enums::Align::Right);
    frame_uplifting_link3.with_label(classified_vec[1][2].headline.as_str());
    let frame_devastating = frame::Frame::new(0,210,50,50,"Devasting Stories").with_align(enums::Align::Right);
    let frame_devastating_link1 = frame::Frame::new(10,230,50,50,"link").with_align(enums::Align::Right);
    frame_devastating_link1.with_label(classified_vec[2][0].headline.as_str());
    let frame_devastating_link2 = frame::Frame::new(10,250,50,50,"link").with_align(enums::Align::Right);
    frame_devastating_link2.with_label(classified_vec[2][1].headline.as_str());
    let frame_devastating_link3 = frame::Frame::new(10,270,50,50,"link").with_align(enums::Align::Right);
    frame_devastating_link3.with_label(classified_vec[2][2].headline.as_str());

    let mut input = input::Input::new(900,150,400,50, "Try it yourself:");
    input.set_label_font(enums::Font::HelveticaBold);
    let mut execbutton = button::Button::new(900,200, 100,50,"Execute");

    let mut frame_classification = frame::Frame::new(900, 300, 100,100, "Our Classification: ");
         
    win.end();
    win.show();
    frame.show();
    app 
    
}

fn classify(newsarticles : Data) -> Vec<Vec<HeadlineURLPair>> {
    //Retrieve headlines from newsarticles
    let headlines = newsarticles.get_headlines();

    let input: Vec<&str> = headlines.iter().map(|s| s.as_ref()).collect();
    let candidate_labels = &["dramatic", "uplifting", "devastating"];
    
    //Calling zeroshotclassification model on headlines
    let sequence_classification_model = ZeroShotClassificationModel::new(Default::default()).unwrap();
    let output = sequence_classification_model.predict(
        input.as_slice(),
        candidate_labels,
        None,
        128,
    );

    let mut sections: Vec<Vec<HeadlineURLPair>> = Vec::new();

    for candidate_label in candidate_labels {
        let mut sec: Vec<HeadlineURLPair> = Vec::new();
        let mut score: f64 = 0.90;

        //Push the highest scores first
        while score >= 0.50 {
        for i in 0..output.len() {
            if &(output[i].text)[..] == *candidate_label && output[i].score > score && output[i].score <= score + 0.10 {
                sec.push(HeadlineURLPair{headline: headlines[i].clone(),url: newsarticles.url_at(i)});
            }
        }
        score -= 0.10;
        }
        
        //Create empty headlines to guarantee atleast 3
        while sec.len() < 3 {
            sec.push(HeadlineURLPair{headline: "".to_string(),url: "".to_string()});
        }
        sections.push(sec);
    }
    sections
}

fn main() { 
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
