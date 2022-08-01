extern crate rustmail;
extern crate diesel;

use self::diesel::prelude::*;
use self::rustmail::*;
use self::models::Mail;
use std::env::args;

fn main() {
    use rustmail::schema::mails::dsl::{mails, published};

    let id = args().nth(1).expect("[rustmail] Requires mail id to update")
        .parse::<i32>().expect("[rustmail] Invalid ID");
    let connection = establish_connection();

    let mail = diesel::update(mails.find(id))
        .set(published.eq(true))
        .get_result::<Mail>(&connection)
        .expect(&format!("[rustmail] Unable to find mail {}", id));
    println!("[rustmail] Updated mail {}", mail.title);
}