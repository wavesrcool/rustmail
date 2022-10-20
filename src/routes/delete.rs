extern crate rustmail;
extern crate diesel;

use self::diesel::prelude::*;
use self::rustmail::*;
use std::env::args;

//
fn main() {
    use rustmail::schema::mails::dsl::*;

    let target = args().nth(1).expect("[rustmail] Requires argument to match mail title");
    let pattern = format!("%{}%", target);

    let connection = establish_connection();
    let num_deleted = diesel::delete(mails.filter(title.like(pattern)))
        .execute(&connection)
        .expect("[rustmail] Error deleting mails");

    println!("[rustmail] Deleted {} mails", num_deleted);
}