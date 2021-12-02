use regex::Regex;

fn main() {
    let input = "U345,T123";
    let  regex = Regex::new(r"\w\d{3}").unwrap();

    for cap in regex.captures_iter(input) {
        println!("User number: {}", &cap[0]);
    }
}
