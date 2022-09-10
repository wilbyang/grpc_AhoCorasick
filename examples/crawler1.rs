use std::io::Read;
use select::document::Document;
use select::predicate::Name;
use std::collections::HashSet;

fn main() {
    let client = reqwest::blocking::Client::new();
    let origin_url = "https://rolisz.ro/";
    let mut res = client.get(origin_url).send().unwrap();
    println!("Status for {}: {}", origin_url, res.status());

    let mut body  = String::new();
    res.read_to_string(&mut body).unwrap();

    let found_urls = Document::from(body.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .map(str::to_string)
        .collect::<HashSet<String>>();
    println!("URLs: {:#?}", found_urls)
}