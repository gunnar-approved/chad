extern crate slack;
extern crate regex;
extern crate hyper;
extern crate serde_json;

mod chad;

use chad::*;
use regex::Regex;


use serde_json::Value;
use serde_json::Map;
use serde_json::value::ToJson;

fn main() {
    let api_key = "KEYHERE";
    let result = r#"{"batchcomplete":"","query":{"redirects":[{"from":"Boobies","to":"Booby"}],"pages":{"223034":{"pageid":223034,"ns":0,"title":"Booby"}}}}"#;
    let v:Value = match serde_json::from_str(result) {
        Ok(response) => response,
        Err(_) => panic!("So close..."),
    };
    let x : Map<String, Value>= v["query"]["pages"].unwrap();
    let best = x.get(&0);
    println!("{}", x);
}

fn main2() {
    let mut chad = Chad::new();
    init(&mut chad);
    let mut cli = slack::RtmClient::new(&api_key);
    let r = cli.login_and_run::<Chad>(&mut chad);
    match r {
        Ok(_) => {}
        Err(err) => panic!("Error: {}", err),
    }
    println!("{}", cli.get_name().unwrap());
    println!("{}", cli.get_team().unwrap().name);
}

// fill chad up
// add database here
fn init(chad: &mut Chad) {
    let re = Regex::new(r"echo").unwrap();
    let handler = EchoHandler::new(re);
    //chad.add_handler(handler);
    let re2 = Regex::new(r"^wiki").unwrap();
    let handler2 = WikiHandler::new(re2);
    chad.add_handler(handler2);
}
