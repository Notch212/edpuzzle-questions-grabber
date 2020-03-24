use serde::Deserialize;
use ureq;

fn main() {
    let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiI1ZDc3OWQxMTZmYjI3NTQwYzU1YTQ0NGEiLCJyb2xlIjoic3R1ZGVudCIsInJlZ2lzdGVyZWRBdCI6MTU2ODEyMDA4MSwiaXNBZG1pbiI6ZmFsc2UsImJlY29tZVRoaXNVc2VyIjpmYWxzZSwiaXNPcGVuQ2xhc3Nyb29tVXNlciI6ZmFsc2UsImlhdCI6MTU4NTA3NDM5NywiZXhwIjoxNTg1Njc5MTk3LCJqdGkiOiI1ZTdhNTBkZGYzYzlkYjNmOTExYzYyYTgifQ.WXrCd8BVcmCPG06cYCST64Af6SwQYw7JdGwk4MvSmAM"; //get this using some type of cookie editor - EditThisCookie or Cookie Editor
    let agent = ureq::Agent::new()
        .set("Cookie", &format!("token={}", token))
        .build();
    let response = agent
        .get("https://edpuzzle.com/api/v3/assignments/5e73a215b35f833e3e663e77")
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