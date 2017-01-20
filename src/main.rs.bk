extern crate slack;
extern crate regex;

mod chad;

use chad::*;
use regex::Regex;

fn main() {
    let api_key = "KEYHERE";
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
    chad.add_handler(handler);
}