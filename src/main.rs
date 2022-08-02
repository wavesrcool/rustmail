extern crate rustmail;


#[macro_use] extern crate rocket;
use rocket::serde::json::Json;

extern crate serde;
use serde::{Serialize, Deserialize};
// Import this crate to derive the Serialize and Deserialize traits.
// #[macro_use] extern crate serde_derive;

extern crate mailgun_rs;
use mailgun_rs::{Mailgun, EmailAddress, Message};

//use std::error::Error;
use regex::Regex;

use dotenv;
use quoted_printable::{decode, ParseMode};

#[macro_use]
extern crate dotenv_codegen;


// * * * * * * * * * * * * * * * * * * *
//  rustmail
//
//  structs "/send"
//
// * * * * * * * * * * * * * * * * * * *
#[derive(Serialize, Deserialize, Debug)]
struct MailsSend {
    root: String,
    to: String,
    //from: String
    body: String,
    subject: String
}

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
struct ResponseMailsSendSuccess(&'static str);

#[derive(Responder)]
#[response(status = 400, content_type = "json")]
struct ResponseMailsSendFailure(&'static str);

// * * * * * * * * * * * * * * * * * * *
//  rustmail
//
//  route: "/send"
//
// * * * * * * * * * * * * * * * * * * *
#[post("/send", format = "json", data = "<send>")]
fn send_mail(send: Json<MailsSend>) {
    println!("{:?} send!", &send);

    let domain = String::from(dotenv!("MAILS_DOMAINS_ORG"));
    let key_org = String::from(dotenv!("MAILS_KEYS_ORG"));
     
    //let domain_io = String::from(dotenv!("MAILS_DOMAINS_IO"));
    //let key_io = String::from(dotenv!("MAILS_KEYS_IO"));
  

    println!("{:?} dom!", domain);
    println!("{:?} key!", key_org);

    let to = EmailAddress::address(&send.to);

    let from_name = String::from("Tyson Lupul");
    let from_address = String::from("tyson@radroots.io");
    let from = EmailAddress::name_address(&from_name, &from_address);


    let body = format!("<h1>{body}</h1>", body = &send.body);

    let message = Message {
        to: vec![to],
        subject: String::from(&send.subject),
        html: body,
        ..Default::default()
    };

    let client = Mailgun{api_key: key_org, domain, message};

    match client.send(&from) {
        Ok(_) => {
          println!("successful");
        }
        Err(err) => {
          println!("{}", err.to_string());
        }
      }

}


// * * * * * * * * * * * * * * * * * * *
//  rustmail
//
//  route: "/receive"
//
// * * * * * * * * * * * * * * * * * * *
#[post("/receive", data = "<receive>")]
fn receive_mail(receive: String) {
    println!("{:?} receive!", receive);

    let decoded = decode(&receive.as_bytes(), ParseMode::Robust).unwrap();


    println!("{:?} decoded!", decoded);

    let letters = String::from_utf8(decoded).unwrap();



    let body_plain_regex = Regex::new(r"^&body-plain=([\s\S]*)&stripped-text=$").unwrap();
    let body_plain_match = body_plain_regex.is_match(&letters);

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


// * * * * * * * * * * * * * * * * * * *
//  rustmail
//
//  struct BreatheResponse
//
// * * * * * * * * * * * * * * * * * * *
#[derive(Responder)]
#[response(status = 200, content_type = "json")]
struct BreatheResponse(&'static str);

// * * * * * * * * * * * * * * * * * * *
//  rustmail
//
//  route: "/breathe"
//
// * * * * * * * * * * * * * * * * * * *
#[get("/breathe")]
fn breathe() -> BreatheResponse {
    BreatheResponse("{ \"rust\": \"mail\" }")
}


// * * * * * * * * * * * * * * * * * * *
//  rustmail
//
//  rocket
//
// * * * * * * * * * * * * * * * * * * *
#[launch]
fn rocket() -> _ {
    dotenv::dotenv().ok();

    rocket::build().mount("/", routes![breathe]).mount("/", routes![receive_mail]).mount("/", routes![send_mail])
}

    //let deserialized:  = serde_json::from_str(&receive).unwrap();
    //println!("deserialized = {:?}", deserialized);
    //println!("deserialized.x = {:?}", deserialized.x);
    //println!("deserialized.y = {:?}", deserialized.y);