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
        "This graph titled ubc vancouver math course enrolment ( fte ) 2010 to \
        2019 2500 shows a rising trend in enrolment in math courses over year. \
        The X-Axis goes from 2010 to 201 8. The Y-Axis goes to 2500. The end of \
        the graph continues to rise and the graph show's few fluctuations.",

        "A screenshot of a cell phone",

        "Image"
    ), (
        "This graph titled monthly passenger count september 2018 - august 2019 \
        shows a rising trend in of passengers over month. The X-Axis goes to 19. \
        The Y-Axis goes from 18 to 18. The end of the graph continues to rise \
        and the graph shows few fluctuations.",

        "A close up of a map",

        "Image"
    ), (
        "This graph titled arsenal team rating average : premier league shows a \
        changing trend in dollars over year. The X-Axis goes to 10. The Y-Axis \
        goes . The end of the graph stabilizing and the graph shows few \
        fluctuations.",

        "A screenshot of a cell phone",

        "Image"
    ), (
        "This graph titled gain required to recover a losing position shows a \
        rising trend in required to breakeven in position over year. The X-Axis \
        goes to 100. The Y-Axis goes . The end of the graph continues to rise \
        and the graph shows few fluctuations.",

        "A close up of text on a black background",

        "Image"
    ), (
        "This graph titled weight shows a changing trend in weight over year. \
        The X-Axis goes to 19. The Y-Axis goes . The end of the graph is falling \
        and the graph shows many fluctuations.",

        "A close up of a map",

        "Image"
    ), (
        "This graph titled daily stock market flunctuation index shows a \
        changing trend in index over month. The X-Axis goes to 19. The Y-Axis \
        goes to 4000. The end of the graph is rising and the graph shows many \
        fluctuations.",

        "A close up of a map",

        "Image"
    ), (
        "This graph titled snapchat / instagram stories of dog shows a rising \
        trend in dollars over year. The X-Axis goes . The Y-Axis goes . The end \
        of the graph continues to rise and the graph shows few fluctuations.",

        "A screenshot of a cell phone",

        "Image"
    ), (
        "This graph titled federal surplus or deficit [ - ] shows a rising trend \
        in of dollars over year. The X-Axis goes from 2010 to 2017. The Y-Axis \
        goes from 400 to 400. The end of the graph is falling and the graph \
        shows few fluctuations.",

        "A screenshot of a cell phone",

        "Image"
    ), (
        "This graph titled unemployment rate for african americans , 1972 - 2019 \
        shows a changing trend in rate over year. The X-Axis goes from 1972 to \
        2017. The Y-Axis goes . The end of the graph is rising and the graph \
        shows many fluctuations.",

        "A close up of a map",

        "Image"
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
