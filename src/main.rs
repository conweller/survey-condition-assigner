#![feature(proc_macro_hygiene, decl_macro)]
use rand::thread_rng;
use rand::seq::SliceRandom;
use rocket_contrib::json::Json;
#[macro_use] extern crate serde_derive;

#[macro_use] extern crate rocket;

#[derive(Serialize, Deserialize)]
struct MyResponse {
    tweet_assignments: Vec<Assignment>
}

#[derive(Serialize, Deserialize)]
struct Assignment {
    source: String,
    alt_text: String,
}

#[get("/")]
fn assignment() -> Json<MyResponse> {
    let response = MyResponse{
        tweet_assignments: get_assignment(ALT_TEXTS.to_vec()),
    };
    Json(response)
        
}

enum Condition {
    Ours,
    Microsoft,
    Control,
}

const ALT_TEXTS: [(&str, &str, &str); 9] = [
    (
        "our alt_text 1",
        "microsoft's alt_text 1",
        "control alt_text 1"
    ), (
        "our alt_text 2",
        "microsoft's alt_text 2",
        "control alt_text 2"
    ), (
        "our alt_text 3",
        "microsoft's alt_text 3",
        "control alt_text 3"
    ), (
        "our alt_text 4",
        "microsoft's alt_text 4",
        "control alt_text 4"
    ), (
        "our alt_text 5",
        "microsoft's alt_text 5",
        "control alt_text 5"
    ), (
        "our alt_text 6",
        "microsoft's alt_text 6",
        "control alt_text 6"
    ), (
        "our alt_text 7",
        "microsoft's alt_text 7",
        "control alt_text 7"
    ), (
        "our alt_text 8",
        "microsoft's alt_text 8",
        "control alt_text 8"
    ), (
        "our alt_text 9",
        "microsoft's alt_text 9",
        "control alt_text 9"
    ),
];

fn get_assignment(alt_texts: Vec<(&str, &str, &str)>) -> Vec<Assignment> {

    let mut rng = thread_rng();
    let mut conditions: Vec<&Condition> = [
        Condition::Ours,
        Condition::Microsoft,
        Condition::Control,
    ]
        .iter()
        .cycle()
        .take(alt_texts.len())
        .collect();

    conditions.shuffle(&mut rng);

    conditions
        .iter()
        .zip(alt_texts.iter())
        .map(|(cond, alt)| {
            match cond {
                Condition::Ours => return Assignment {
                    source: String::from("Ours"),
                    alt_text: String::from(alt.0)
                },
                Condition::Microsoft => return Assignment {
                    source: String::from("Microsoft"),
                    alt_text: String::from(alt.1)

                },
                Condition::Control => return Assignment {
                    source: String::from("Control"),
                    alt_text: String::from(alt.2)
                },
            }
        })
    .collect()
}

fn main() {
    rocket::ignite().mount("/assignment", routes![assignment]).launch();
}
