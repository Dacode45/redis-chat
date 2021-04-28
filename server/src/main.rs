#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate serde;

use rocket::State;
use rocket_contrib::json::Json;

use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use std::sync::{Arc, Mutex};

mod unique_id;
use unique_id::UniqueId;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[allow(non_snake_case)] // Cheat so json is in the form we care about
pub struct Offer {
    pub candidate: String,
    pub sdpMLineIndex: i32,
    pub sdpMid: String,
    pub usernameFragment: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct Call {
    pub answer_candidates: HashMap<UniqueId, Offer>,
    pub offer_candidates: HashMap<UniqueId, Offer>,
}

type DB = Arc<Mutex<HashMap<UniqueId, Call>>>;

fn main() {
    rocket::ignite()
        .manage(DB::default())
        .mount(
            "/",
            routes![
                index,
                call,
                get_call,
                get_calls,
                answer,
                get_answser,
                offer,
                get_offer
            ],
        )
        .launch();
}

#[get("/")]
fn index() -> &'static str {
    "
    USAGE

      POST /

          accepts raw data in the body of the request and responds with a URL of
          a page containing the body's content

      GET /<id>

          retrieves the content for the paste with id `<id>`
    "
}

#[post("/calls")]
fn call(state: State<DB>) -> Json<String> {
    let id = UniqueId::new(3);
    let mut state = state.lock().unwrap();
    state.insert(id.clone(), Call::default());

    // let url = format!("{host}/call{id}\n", host = "http://localhost:8000", id = id);

    // Write the paste out to the file and return the URL.
    Json(id.0.clone())
}

#[get("/calls")]
fn get_calls(state: State<DB>) -> Json<HashMap<UniqueId, Call>> {
    let state = state.lock().unwrap();

    // let url = format!("{host}/call{id}\n", host = "http://localhost:8000", id = id);

    // Write the paste out to the file and return the URL.
    Json(state.clone())
}

#[get("/calls/<call_id>")]
fn get_call(state: State<DB>, call_id: UniqueId) -> Option<Json<Call>> {
    let state = state.lock().unwrap();
    if let Some(call) = state.get(&call_id) {
        return Some(Json(call.clone()));
    } else {
        return None;
    }
}

#[post("/calls/<call_id>/answers")]
fn answer(state: State<DB>, call_id: UniqueId) -> Option<Json<String>> {
    let id = UniqueId::new(3);
    let mut state = state.lock().unwrap();
    if let Some(call) = state.get_mut(&call_id) {
        call.answer_candidates.insert(id.clone(), Offer::default());
        println!("{:?}", call);
        return Some(Json(id.0.clone()));
    } else {
        return None;
    }
}

#[get("/calls/<call_id>/answers/<answer_id>")]
fn get_answser(state: State<DB>, call_id: UniqueId, answer_id: UniqueId) -> Option<Json<Offer>> {
    let state = state.lock().unwrap();
    if let Some(call) = state.get(&call_id) {
        return call
            .answer_candidates
            .get(&answer_id)
            .map(|offer| Json(offer.clone()));
    } else {
        return None;
    }
}

#[post("/calls/<call_id>/offers")]
fn offer(state: State<DB>, call_id: UniqueId) -> Option<Json<String>> {
    let id = UniqueId::new(3);
    let mut state = state.lock().unwrap();
    if let Some(call) = state.get_mut(&call_id) {
        call.offer_candidates.insert(id.clone(), Offer::default());
        return Some(Json(id.0.clone()));
    } else {
        return None;
    }
}

#[get("/calls/<call_id>/offers/<offer_id>")]
fn get_offer(state: State<DB>, call_id: UniqueId, offer_id: UniqueId) -> Option<Json<Offer>> {
    let state = state.lock().unwrap();
    if let Some(call) = state.get(&call_id) {
        return call
            .offer_candidates
            .get(&offer_id)
            .map(|offer| Json(offer.clone()));
    } else {
        return None;
    }
}
