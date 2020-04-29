#![feature(proc_macro_hygiene, decl_macro)]
use rand::thread_rng;
use rand::seq::SliceRandom;
use rocket_contrib::json::Json;
#[macro_use] extern crate serde_derive;

#[macro_use] extern crate rocket;


const NUM_BLOCKS: usize = 9;

#[derive(Serialize, Deserialize)]
struct MyResponse {
    contents: Vec<(String, String)>
}


#[get("/")]
fn assignment() -> Json<MyResponse> {
    let response = MyResponse{
        contents: get_assignment(),
    };
    Json(response)
        
}

enum Condition {
    Ours,
    Microsoft,
    Control,
}

const DESCRIPTIONS: [(&str, &str, &str); NUM_BLOCKS] = [
    (
        "our description 1",
        "microsoft's description 1",
        "control description 1"
    ), (
        "our description 2",
        "microsoft's description 2",
        "control description 2"
    ), (
        "our description 3",
        "microsoft's description 3",
        "control description 3"
    ), (
        "our description 4",
        "microsoft's description 4",
        "control description 4"
    ), (
        "our description 5",
        "microsoft's description 5",
        "control description 5"
    ), (
        "our description 6",
        "microsoft's description 6",
        "control description 6"
    ), (
        "our description 7",
        "microsoft's description 7",
        "control description 7"
    ), (
        "our description 8",
        "microsoft's description 8",
        "control description 8"
    ), (
        "our description 9",
        "microsoft's description 9",
        "control description 9"
    ),
];

fn get_assignment() -> Vec<(String, String)> {

    let mut rng = thread_rng();
    let mut conditions: Vec<&Condition> = [
        Condition::Ours,
        Condition::Microsoft,
        Condition::Control,
    ]
        .iter()
        .cycle()
        .take(NUM_BLOCKS)
        .collect();

    conditions.shuffle(&mut rng);

    conditions
        .iter()
        .zip(DESCRIPTIONS.iter())
        .map(|(cond, desc)| {
            match cond {
                Condition::Ours => return (
                    String::from("Ours"),
                    String::from(desc.0)
                ),
                Condition::Microsoft => return (
                    String::from("Microsoft"),
                    String::from(desc.1)
                ),
                Condition::Control => return (
                    String::from("Control"),
                    String::from(desc.2)
                ),
            }
        })
    .collect()
}

fn main() {
    rocket::ignite().mount("/assignment", routes![assignment]).launch();
}
