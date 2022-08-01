pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use self::models::{Mail, NewMail};

pub fn add_mail<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> Mail {
    use schema::mails;

    let new_mail = NewMail {
        title: title,
        body: body,
    };

    diesel::insert_into(mails::table)
        .values(&new_mail)
        .get_result(conn)
        .expect("Error saving new mail")
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}