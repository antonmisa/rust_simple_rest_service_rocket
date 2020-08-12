use crate::rocket;
use rocket::local::Client;
use rocket::http::{Status, ContentType};

#[test]
fn test_status() {	
	let client = Client::new(rocket()).unwrap();
	let mut resp = client.get("/status").dispatch();
	assert_eq!(resp.status(), Status::Ok);
	
	let response_body = resp.body_string().unwrap();
    assert_eq!(response_body, r##"{"code":0,"description":"Everything is OK!","result":true}"##);
}

#[test]
fn test_get() {	
	let client = Client::new(rocket()).unwrap();
	let mut resp = client.get("/data/v1/test").dispatch();
	assert_eq!(resp.status(), Status::Ok);
	
	let response_body = resp.body_string().unwrap();
    assert_eq!(response_body, r##"{"code":0,"description":"You requested get method with name: test","result":true}"##);
}

#[test]
fn test_post() {	
	let client = Client::new(rocket()).unwrap();
	let mut resp = client
						.post("/data/v1/test")
						.header(ContentType::JSON)
						.body(r#"{"data":"test data"}"#)
						.dispatch();
	assert_eq!(resp.status(), Status::Ok);
	
	let response_body = resp.body_string().unwrap();
    assert_eq!(response_body, r##"{"code":0,"description":"You requested post method with name: test, data is test data","result":true}"##);
}

#[test]
fn test_put() {	
	let client = Client::new(rocket()).unwrap();
	let mut resp = client
						.put("/data/v1/test")
						.header(ContentType::JSON)
						.body(r#"{"data":"test data"}"#)
						.dispatch();
	assert_eq!(resp.status(), Status::Ok);
	
	let response_body = resp.body_string().unwrap();
    assert_eq!(response_body, r##"{"code":0,"description":"You requested put method with name: test, data is test data","result":true}"##);
}

#[test]
fn test_delete() {	
	let client = Client::new(rocket()).unwrap();
	let mut resp = client
						.delete("/data/v1/test")
						.header(ContentType::JSON)
						.body(r#"{"data":"test data"}"#)
						.dispatch();
	assert_eq!(resp.status(), Status::Ok);
	
	let response_body = resp.body_string().unwrap();
    assert_eq!(response_body, r##"{"code":0,"description":"You requested delete method with name: test, data is test data","result":true}"##);
}