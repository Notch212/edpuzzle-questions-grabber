use serde::Deserialize;
use ureq;

fn main() {
    let token = "------"; //get this using some type of cookie editor - EditThisCookie or Cookie Editor
    let agent = ureq::Agent::new()
        .set("Cookie", &format!("token={}", token))
        .build();
    let response = agent
        .get("https://edpuzzle.com/api/v3/assignments/----") //replace --- with the assignment id thing, should be obvious
        .call()
        .into_json()
        .unwrap();
    //0 is hardcoded
    let questions_vec = response["medias"][0]["questions"].clone();
    let questions: Vec<QuestionsData> = serde_json::from_value(questions_vec).unwrap();
    for question in questions {
        //0 is hardcoded
        let question_body = &question.body[0];
        if question_body.html.len() == 0 { //if there is no html text, read the normal text
            println!("question text: {}", question_body.text);
        } else { //print the html text
            println!("question html: {}", question_body.html);
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