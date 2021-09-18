#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use serde_json::from_slice;

#[macro_use]
extern crate lazy_static;
extern crate mut_static;

use mut_static::MutStatic;

#[derive(FromForm, Serialize, Deserialize)]
struct Request {
    code: String,
    hash: String,
}

#[derive(Deserialize, Serialize)]
struct Response {
    is_allowed: bool,
}


lazy_static! {
    static ref GLOBAL: MutStatic<HashMap<String, String>> = MutStatic::from(HashMap::new());
}

const FILE_NAME: &str = "cred.txt";

#[post("/", data = "<credentials>")]
fn index(credentials: Json<Request>) -> String {
    let mut cred: Vec<u8> = Vec::new();
    { 
        let mut file = File::open(FILE_NAME).expect("Error in file openning");
        let _ = file.read_to_end(&mut cred);
    }

    let credentials = credentials.into_inner();
    let mut response = Response { is_allowed: false };
    let mut global: HashMap<String, String> = from_slice(cred.as_slice()).expect("Error in deserialize cred");
    if let Some(hash) = global.get(&credentials.code) {
        if *hash == credentials.hash {
            response.is_allowed = true;
        }
    } else {
        global.insert(credentials.code, credentials.hash);
        response.is_allowed = true;
        let mut file = File::create(FILE_NAME).expect("Error in file creation");
        file.write_all(serde_json::to_vec_pretty(&global).expect("Error in serialize global").as_slice());
        file.sync_all();
    }

    println!("res {}", serde_json::to_string(&response).unwrap());
    serde_json::to_string(&response).unwrap()
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}