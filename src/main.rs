#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

#[cfg(test)] mod test;

use rocket::config::{Config, Environment};
use rocket_contrib::json::{Json, JsonValue};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct OutputData {
	result: bool,
	code:   u32,
	description: String,
}

#[derive(Serialize, Deserialize)]
struct InputData {
	data: String,
}

#[get("/status")]
fn get_status() -> JsonValue {
    json!( OutputData{result: true, code: 0, description: "Everything is OK!".to_string()} )
}

#[get("/data/v1/<name>")]
fn get_data_v1(name: String) -> JsonValue {
    json!( OutputData {result: true, code: 0, description: format!("You requested get method with name: {}", name)} )
}

#[post("/data/v1/<name>", format="json", data = "<data>")]
fn post_data_v1(name: String, data: Json<InputData>) -> JsonValue {
    json!( OutputData {result: true, code: 0, description: format!("You requested post method with name: {}, data is {}", name, data.data)} )
}

#[put("/data/v1/<name>", format="json", data = "<data>")]
fn put_data_v1(name: String, data: Json<InputData>) -> JsonValue {
    json!( OutputData {result: true, code: 0, description: format!("You requested put method with name: {}, data is {}", name, data.data)} )
}

#[delete("/data/v1/<name>", format="json", data = "<data>")]
fn delete_data_v1(name: String, data: Json<InputData>) -> JsonValue {
    json!( OutputData {result: true, code: 0, description: format!("You requested delete method with name: {}, data is {}", name, data.data)} )
}

fn rocket() -> rocket::Rocket {
	let config = Config::build(Environment::Staging)
		.address("0.0.0.0")
		.port(8000)
		.finalize().expect("Config error");
	
    rocket::custom(config).mount("/", routes![get_status, get_data_v1, post_data_v1, put_data_v1, delete_data_v1])//.launch();
}

fn main() {
	rocket().launch();
}
