extern crate rustmail;
extern crate diesel;

use self::rustmail::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use rustmail::schema::mails;

    let connection = establish_connection();
    let results = mails::table.filter(mails::published.eq(true))
        .limit(5)
        .load::<Mail>(&connection)
        .expect("[rustmail] Error loading mails");

    println!("[rustmail] Displaying {} mails", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}