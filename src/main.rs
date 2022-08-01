#[macro_use] extern crate rocket;
extern crate rustmail;

//use rocket::serde::json::Json;
use serde::{Serialize, Deserialize};

// #[post("/user", format = "json", data = "<user>")]
// fn new_user(user: Json<rustmail::models::Sender>) {
    /* ... */
//}

extern crate serde;
extern crate serde_json;

// Import this crate to derive the Serialize and Deserialize traits.
// #[macro_use] extern crate serde_derive;

#[derive(Serialize, Deserialize, Debug)]
struct Sender {
    x: i32,
    y: i32,
}

#[post("/send", data = "<send>")]
fn send_mails(send: String) {
    println!("{:?} SEND!", send);

    let deserialized: Sender = serde_json::from_str(&send).unwrap();
    println!("deserialized = {:?}", deserialized);
    println!("deserialized.x = {:?}", deserialized.x);
    println!("deserialized.y = {:?}", deserialized.y);


    //println!("point = {}", point);

    // Convert the Point to a JSON string.
    //let serialized = serde_json::to_string(send).unwrap();
   
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index]).mount("/", routes![send_mails])
}