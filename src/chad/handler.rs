use regex::Regex;
use hyper::Client;
use hyper::header::Connection;
use std::io::Read;
use serde_json::Value;

pub trait MessageHandler {
    fn handle(&self, message: &str) -> Option<String>;
    fn new(regex: Regex) -> Box<MessageHandler> where Self: Sized;
}

pub struct EchoHandler {
    regex: Regex,
}

pub struct WikiHandler {
    regex: Regex,
}


impl MessageHandler for EchoHandler {
    fn new(regex: Regex) -> Box<MessageHandler> {
        Box::new(EchoHandler { regex: regex })
    }

    fn handle(&self, message: &str) -> Option<String> {
        if self.regex.is_match(message) {
            return Some(message.to_owned());
        }

        return None;
    }
}


impl MessageHandler for WikiHandler {
    fn new(regex: Regex) -> Box<MessageHandler> {
        Box::new(WikiHandler { regex: regex })
    }

    fn handle(&self, message: &str) -> Option<String> {
        if self.regex.is_match(message) {
            let thing = &message[5..]; // everything after wiki, I know is not clean
            let url = "https://en.wikipedia.org/w/api.php?action=query&format=json&redirects=1&utf8=1&titles=".to_string()+thing;

            let client = Client::new();
            let mut res = client.get(&*url)
                .header(Connection::close())
                .send().unwrap();

            println!("Response: {}", res.status);
            println!("Headers:\n{}\n", res.headers);
            let mut content = String::new();
            res.read_to_string(&mut content);
            println!("Resp:\n{}", content);



            return Some(content.to_owned());
        }

        return None;
    }


}