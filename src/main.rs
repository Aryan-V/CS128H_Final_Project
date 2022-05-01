pub mod lib;

use lib::Data;
use rust_bert::pipelines::sequence_classification::Label;
use rust_bert::pipelines::zero_shot_classification::ZeroShotClassificationModel;
use fltk::{app, prelude::*, window};
use fltk::*;

pub struct HeadlineURLPair {
    headline: String,
    url: String,
}

fn create_app(classified_vec: &Vec<Vec<HeadlineURLPair>>) -> fltk::app::App {
    //Set the dimensions of the window
    const WIDTH: i32 = 1800;
    const HEIGHT: i32 = 1000;

    let app = app::App::default().with_scheme(app::Scheme::Plastic);
    let mut win = window::Window::default().with_size(WIDTH, HEIGHT).center_screen();

    //Left side of window
    let mut l_pack = group::Pack::new(0,0,WIDTH/2, HEIGHT, "");
    let mut recommended_reads_frame = frame::Frame::new(80,0,50,50,"Recommended Reads Today");
    recommended_reads_frame.set_label_font(enums::Font::HelveticaBold);
    
    //List of top three dramatic articles 
    let mut frame_dramatic = frame::Frame::new(0,30,50,50,"Dramatic Stories");
    frame_dramatic.set_label_font(enums::Font::HelveticaBold);
    let frame_dramatic_link1 = frame::Frame::new(10,50,50,50,"link");
    frame_dramatic_link1.with_label(&(classified_vec[0][0].headline.as_str().to_owned() + "\n" + classified_vec[0][0].url.as_str()));
    let frame_dramatic_link2 = frame::Frame::new(10,70,50,50, "link");
    frame_dramatic_link2.with_label(&(classified_vec[0][1].headline.as_str().to_owned() + "\n" + classified_vec[0][1].url.as_str()));
    let frame_dramatic_link3 = frame::Frame::new(10,90,50,50,"link");
    frame_dramatic_link3.with_label(&(classified_vec[0][2].headline.as_str().to_owned() + "\n" + classified_vec[0][2].url.as_str()));
    
    //List of top three uplifting articles
    let mut frame_uplifting = frame::Frame::new(80,120,50,50,"Uplifting Stories");
    frame_uplifting.set_label_font(enums::Font::HelveticaBold);
    let frame_uplifting_link1 = frame::Frame::new(10,140,50,50,"link");
    frame_uplifting_link1.with_label(&(classified_vec[1][0].headline.as_str().to_owned() + "\n" + classified_vec[1][0].url.as_str()));
    let frame_uplifting_link2 = frame::Frame::new(10,160,50,50,"link");
    frame_uplifting_link2.with_label(&(classified_vec[1][1].headline.as_str().to_owned() + "\n" + classified_vec[1][1].url.as_str()));
    let frame_uplifting_link3 = frame::Frame::new(10,180,50,50,"link");
    frame_uplifting_link3.with_label(&(classified_vec[1][2].headline.as_str().to_owned() + "\n" + classified_vec[1][2].url.as_str()));

    //List of top three informative articles
    let mut frame_informative = frame::Frame::new(80,210,50,50,"Informative Stories");
    frame_informative.set_label_font(enums::Font::HelveticaBold);
    let frame_informative_link1 = frame::Frame::new(10,230,50,50,"link");
    frame_informative_link1.with_label(&(classified_vec[2][0].headline.as_str().to_owned() + "\n" + classified_vec[2][0].url.as_str()));
    let frame_informative_link2 = frame::Frame::new(10,250,50,50,"link");
    frame_informative_link2.with_label(&(classified_vec[2][1].headline.as_str().to_owned() + "\n" + classified_vec[2][1].url.as_str()));
    let frame_informative_link3 = frame::Frame::new(10,270,50,50,"link");
    frame_informative_link3.with_label(&(classified_vec[2][2].headline.as_str().to_owned() + "\n" + classified_vec[2][2].url.as_str()));
    
    l_pack.end();
    l_pack.set_type(group::PackType::Vertical);
    
    //Right side of window
    let mut r_pack = group::Pack::new(WIDTH/2, 0, WIDTH/2, HEIGHT, "");

    //Input box and button for users to try out themselves
    let mut input = input::Input::new(0,0,400,50, "Try it yourself:");
    input.set_label_font(enums::Font::HelveticaBold);
    let mut button = button::Button::new(0,0, 100,50,"Submit");
    let mut frame_classification = frame::Frame::new(0, 500, 200 ,100 , "Our Classification: ");

    r_pack.end();
    r_pack.set_type(group::PackType::Vertical);

    win.end();
    win.show();

    //Updates the frame when submit button is pressed
    button.set_callback(move |_| {
        let output = classify_headline(input.value().as_str());
            
        let mut to_display = "Our Classification: \nLabel: ".to_string();
        to_display += &output.text.to_string();
        to_display += "\n Score: ";
        to_display += &output.score.to_string();

        frame_classification.set_label(to_display.as_str());
    });

    app 
}

fn classify(newsarticles : Data) -> Vec<Vec<HeadlineURLPair>> {
    //Retrieve headlines from newsarticles
    let headlines = newsarticles.get_headlines();

    let input: Vec<&str> = headlines.iter().map(|s| s.as_ref()).collect();
    let candidate_labels = &["dramatic", "uplifting", "informative"];
    
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
        
        //Create empty headlines to guarantee at least 3
        while sec.len() < 3 {
            sec.push(HeadlineURLPair{headline: "".to_string(),url: "".to_string()});
        }
        sections.push(sec);
    }
    sections
}

fn classify_headline(headline: &str) -> Label {
    let candidate_labels = &["dramatic", "uplifting", "informative"];
    
    //Calling zeroshotclassification model on headline
    let sequence_classification_model = ZeroShotClassificationModel::new(Default::default()).unwrap();
    let output = sequence_classification_model.predict(
        [headline],
        candidate_labels,
        None,
        128,
    );

    //returns label
    output[0].clone()
}

fn main() { 
    //Calling API to get list of news articles
    let response = lib::retrieve_news_articles();
    let data: Data = match response {
        Ok(data) => data,
        _ => panic!("Did not find data")
    };

    //Running the news article throught the zeroshotclassification model
    let classified_vec: Vec<Vec<HeadlineURLPair>> = classify(data);

    //creating the app to display the results
    let app = create_app(&classified_vec);
    app.run().unwrap();

}
