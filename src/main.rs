extern crate rustmail;

#[macro_use]
extern crate rocket;
use rocket::serde::json::Json;

extern crate serde;
use serde::{Deserialize, Serialize};
// Import this crate to derive the Serialize and Deserialize traits.
// #[macro_use] extern crate serde_derive;

extern crate mailgun_rs;
use mailgun_rs::{EmailAddress, Mailgun, Message};
use urlencoding::decode;
//use std::error::Error;

use dotenv;

#[macro_use]
extern crate dotenv_codegen;

use regex::Regex;

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
    subject: String,
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

    let client = Mailgun {
        api_key: key_org,
        domain,
        message,
    };

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

    let messageid_regex = Regex::new(r"&Message-Id=.*&Mime-Version=").unwrap();
    let messageid_matched = messageid_regex.is_match(&receive);
    let messageid_list: Vec<&str> = messageid_regex
        .find_iter(&receive)
        .map(|x| x.as_str())
        .collect();
    let messageid_found = String::from(messageid_list[0]);

    let messageid_index_start = String::from("&Message-Id=").len();
    let messageid_index_end_abs = String::from("&Mime-Version=").len();
    let messageid_index_end = messageid_found.len() - messageid_index_end_abs;
    let message_id_slice = &messageid_found[messageid_index_start..messageid_index_end];
    let messageid_decoded = decode(&message_id_slice).expect("UTF-8");

    println!("{:?} messageid_matched!", messageid_matched);
    println!("{:?} messageid_decoded!", messageid_decoded);
    println!("");

    //@ todo old pattern was: &sender=.*&subject=
    let sender_regex = Regex::new(r"&sender=.*&signature=").unwrap();
    let sender_matched = sender_regex.is_match(&receive);
    let sender_list: Vec<&str> = sender_regex
        .find_iter(&receive)
        .map(|x| x.as_str())
        .collect();
    let sender_found = String::from(sender_list[0]);

    let sender_index_start = String::from("&sender=").len();
    let sender_index_end_abs = String::from("&signature=").len();
    let sender_index_end = sender_found.len() - sender_index_end_abs;
    let message_id_slice = &sender_found[sender_index_start..sender_index_end];
    let sender_decoded = decode(&message_id_slice).expect("UTF-8");

    println!("{:?} sender_matched!", sender_matched);
    println!("{:?} sender_decoded!", sender_decoded);
    println!("");

    let recipient_regex = Regex::new(r"&recipient=.*&sender=").unwrap();
    let recipient_matched = recipient_regex.is_match(&receive);
    let recipient_list: Vec<&str> = recipient_regex
        .find_iter(&receive)
        .map(|x| x.as_str())
        .collect();
    let recipient_found = String::from(recipient_list[0]);

    let recipient_index_start = String::from("&recipient=").len();
    let recipient_index_end_abs = String::from("&sender=").len();
    let recipient_index_end = recipient_found.len() - recipient_index_end_abs;
    let message_id_slice = &recipient_found[recipient_index_start..recipient_index_end];
    let recipient_decoded = decode(&message_id_slice).expect("UTF-8");

    println!("{:?} recipient_matched!", recipient_matched);
    println!("{:?} recipient_decoded!", recipient_decoded);
    println!("");

    let subject_regex = Regex::new(r"&Subject=.*&To=").unwrap();
    let subject_matched = subject_regex.is_match(&receive);
    let subject_list: Vec<&str> = subject_regex
        .find_iter(&receive)
        .map(|x| x.as_str())
        .collect();
    let subject_found = String::from(subject_list[0]);

    let subject_index_start = String::from("&Subject=").len();
    let subject_index_end_abs = String::from("&To=").len();
    let subject_index_end = subject_found.len() - subject_index_end_abs;
    let message_id_slice = &subject_found[subject_index_start..subject_index_end];
    let subject_decoded = decode(&message_id_slice).expect("UTF-8");

    println!("{:?} subject_matched!", subject_matched);
    println!("{:?} subject_decoded!", subject_decoded);
    println!("");

    let timestamp_regex = Regex::new(r"&timestamp=.*&token=").unwrap();
    let timestamp_matched = timestamp_regex.is_match(&receive);
    let timestamp_list: Vec<&str> = timestamp_regex
        .find_iter(&receive)
        .map(|x| x.as_str())
        .collect();
    let timestamp_found = String::from(timestamp_list[0]);

    let timestamp_index_start = String::from("&timestamp=").len();
    let timestamp_index_end_abs = String::from("&token=").len();
    let timestamp_index_end = timestamp_found.len() - timestamp_index_end_abs;
    let message_id_slice = &timestamp_found[timestamp_index_start..timestamp_index_end];
    let timestamp_decoded = decode(&message_id_slice).expect("UTF-8");

    println!("{:?} timestamp_matched!", timestamp_matched);
    println!("{:?} timestamp_decoded!", timestamp_decoded);
    println!("");

    let strippedtext_regex = Regex::new(r"&stripped-text=.*&subject=").unwrap();
    let strippedtext_matched = strippedtext_regex.is_match(&receive);
    let strippedtext_list: Vec<&str> = strippedtext_regex
        .find_iter(&receive)
        .map(|x| x.as_str())
        .collect();
    let strippedtext_found = String::from(strippedtext_list[0]);

    let strippedtext_index_start = String::from("&stripped-text=").len();
    let strippedtext_index_end_abs = String::from("&subject=").len();
    let strippedtext_index_end = strippedtext_found.len() - strippedtext_index_end_abs;
    let message_id_slice = &strippedtext_found[strippedtext_index_start..strippedtext_index_end];
    let strippedtext_decoded = decode(&message_id_slice).expect("UTF-8");

    println!("{:?} strippedtext_matched!", strippedtext_matched);
    println!("{:?} strippedtext_decoded!", strippedtext_decoded);
    println!("");

    /*let message = models::Message {
        m_id: String::from(&messageid_found),
        m_0: String::from(&subject_found),
        m_1: String::from(&strippedtext_found),
        ts: String::from(&messageid_found),
    };

    println!("{:?} message!", message);*/

    //let body_plain_regex = Regex::new(r"&body-plain=([\s\S]*)&stripped-text=").unwrap();
    //let body_plain_match = body_plain_regex.is_match(&receive);
    //let body_plain_result: Vec<&str> = body_plain_regex.find_iter(&receive).map(|x| x.as_str()).collect();

    // let outcome = return if body_plain_match && body_plain_result[1] {true} else {false};

    //println!("{:?} decoded!", letters);

    //let letters String::from(receive)

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

    rocket::build()
        .mount("/", routes![breathe])
        .mount("/", routes![receive_mail])
        .mount("/", routes![send_mail])
}

//let deserialized:  = serde_json::from_str(&receive).unwrap();
//println!("deserialized = {:?}", deserialized);
//println!("deserialized.x = {:?}", deserialized.x);
//println!("deserialized.y = {:?}", deserialized.y);
