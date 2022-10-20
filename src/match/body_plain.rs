pub mod body_plain;

use regex::Regex;

fn quoted_printable_body_plain() {
    let body_plain_regex = Regex::new(r"&body-plain=([\s\S]*)&stripped-text=").unwrap();
    let body_plain_match = body_plain_regex.is_match(&receive);
    let body_plain_result: Vec<&str> = body_plain_regex.find_iter(&receive).map(|x| x.as_str()).collect();

    println!("body_plain_match = {:?}", body_plain_match);

}