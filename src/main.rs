#[macro_use] extern crate rocket;
extern crate rustmail;

//use core::any::Any;
use rocket::serde::json::Json;
use serde::{Serialize, Deserialize};

// #[post("/user", format = "json", data = "<user>")]
// fn new_user(user: Json<rustmail::models::Sender>) {
    /* ... */
//}

extern crate serde;
use regex::Regex;
// Import this crate to derive the Serialize and Deserialize traits.
// #[macro_use] extern crate serde_derive;

#[derive(Serialize, Deserialize, Debug)]
struct MailsSend {
    to: String,
    from: String
}

#[post("/send", format = "json", data = "<send>")]
fn send_mail(send: Json<MailsSend>) {
    println!("{:?} send!", &send);

    let to = &send.to;
    let from = &send.from;
   
    println!("{:?} to!", to);
    println!("{:?} from!", from);

}

#[post("/receive", data = "<receive>")]
fn receive_mail(receive: String) {
    println!("{:?} receive!", receive);

    //let deserialized:  = serde_json::from_str(&receive).unwrap();
    //println!("deserialized = {:?}", deserialized);
    //println!("deserialized.x = {:?}", deserialized.x);
    //println!("deserialized.y = {:?}", deserialized.y);

    let body_plain_regex = Regex::new(r"^&body-plain=([\s\S]*)&stripped-text=$").unwrap();
    let body_plain_match = body_plain_regex.is_match(&receive);

    println!("body_plain_match = {:?}", body_plain_match);

    if body_plain_match { 
        let body_plain_result: Vec<&str> = body_plain_regex.find_iter(&receive).map(|x| x.as_str()).collect();
        println!("body_plain_result = {:?}", body_plain_result);

    }

    //println!("{:?}", result1);
    //println!("{:?}", result1[1]);

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
    rocket::build().mount("/", routes![index]).mount("/", routes![receive_mail]).mount("/", routes![send_mail])
}