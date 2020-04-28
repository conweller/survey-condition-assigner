#![feature(proc_macro_hygiene, decl_macro)]
use rand::thread_rng;
use rand::seq::SliceRandom;
use rocket_contrib::json::Json;
#[macro_use] extern crate serde_derive;

#[macro_use] extern crate rocket;


const NUM_BLOCKS: usize = 9;

#[derive(Serialize, Deserialize)]
struct MyResponse {
    contents: Vec<String>
}


#[get("/")]
fn assignment() -> Json<MyResponse> {
    let response = MyResponse{
        contents: get_assignment(),
    };
    Json(response)
        
}

fn get_assignment() -> Vec<String> {
    let mut rng = thread_rng();
    let mut conditions: Vec<String> = ["us", "ms", "ctrl"]
        .iter()
        .cycle()
        .take(NUM_BLOCKS)
        .map(|x| String::from(*x))
        .collect();

    conditions.shuffle(&mut rng);

    conditions
}

fn main() {
    rocket::ignite().mount("/assignment", routes![assignment]).launch();
}
