use regex::Regex;

pub trait MessageHandler {
    fn handle(&self, message: &str) -> Option<String>;
    fn new(regex: Regex) -> Box<MessageHandler> where Self: Sized;
}

pub struct EchoHandler {
    regex: Regex,
}

impl MessageHandler for EchoHandler {
    fn new(regex: Regex) -> Box<MessageHandler> {
        Box::new(EchoHandler {
            regex: regex,
        })
    }
    fn handle(&self, message: &str) -> Option<String> {
        if self.regex.is_match(message) {
            return Some(message.to_owned());
        }

        return None
    }
}