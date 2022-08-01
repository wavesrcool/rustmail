use regex::Regex;

fn main() {
    //let regex = Regex::new(r"\d{3}").unwrap();
    let a: &str = "123bbasfsdf23asd2021-06-17";
    let result1: Vec<&str> = Regex::new(r"\d{1,3}").unwrap().find_iter(a).map(|x| x.as_str()).collect();
    println!("{:?}", result1);
    println!("{:?}", result1[1]);
}