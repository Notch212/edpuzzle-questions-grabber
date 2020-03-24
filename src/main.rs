use serde::Deserialize;
use ureq;
use std::io;

fn main() {
    let mut token: String = String::new(); //get this using some type of cookie editor - EditThisCookie or Cookie Editor
    let mut magic_hash: String = String::new();
    println!("enter cookie for token: ");
    io::stdin().read_line(&mut token).expect("unable to read line");
    println!("enter hash: ");
    io::stdin().read_line(&mut magic_hash).expect("unable to read line");

    let api_url = format!("https://edpuzzle.com/api/v3/assignments/{}", magic_hash);

    let agent = ureq::Agent::new()
        .set("Cookie", &format!("token={}", token))
        .build();
    let response = agent
        .get(&api_url)
        .call()
        .into_json()
        .unwrap();
    //0 is hardcoded
    let questions_vec = response["medias"][0]["questions"].clone();
    let questions: Vec<QuestionsData> = serde_json::from_value(questions_vec).unwrap();
    for question in questions {
        //0 is hardcoded
        let question_body = &question.body[0];
        if question_body.text.len() == 0 { //if there is no normal text, read the html text
            println!("question html: {}", question_body.html);
        } else { //print the text
            println!("question text: {}", question_body.text);
        }
    }
}

#[derive(Debug, Deserialize)]
struct QuestionBody {
    text: String,
    html: String,
    _id: String,
}

#[allow(non_snake_case)] //the fields in this struct are required to be capitalized like this
#[derive(Debug, Deserialize)]
struct QuestionsData {
    r#type: String, //type is already a reserved keyword in rust
    questionNumber: u32,
    time: f64,
    duration: u32,
    _id: String,
    absoluteTime: f64,
    body: Vec<QuestionBody>,
}