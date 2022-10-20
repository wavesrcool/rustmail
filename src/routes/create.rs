extern crate rustmail;
extern crate diesel;

use self::rustmail::*;
use std::env::args;

//

fn main() {
    let connection = establish_connection();

    let title = args().nth(1).expect("[rustmail] Requires mail title")
        .parse::<String>().expect("[rustmail] Invalid title");

    let body = args().nth(1).expect("[rustmail] Requires mail title")
        .parse::<String>().expect("[rustmail] Invalid title");

    let mail = add_mail(&connection, &title, &body);
    println!("\nSaved draft {} with id {}", title, mail.id);
}