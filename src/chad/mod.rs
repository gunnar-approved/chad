use slack::{EventHandler, RtmClient, Event, Error, Message};

pub mod handler;
pub use self::handler::*;

pub struct Chad {
    handlers: Vec<Box<MessageHandler>>,
}

impl Chad {
    pub fn new() -> Self {
        Chad {
            handlers: Vec::new(),
        }
    }

    pub fn add_handler(&mut self, handler: Box<MessageHandler>) {
        self.handlers.push(handler);
    }

    fn handle_message(&mut self, cli: &mut RtmClient, channel: &str, message: &str) {
        for handler in &self.handlers {
            let result = handler.handle(message);
            if let Some(response) = result {
                let _ = cli.send_message(channel, &response);
            }
        }
    }
}

impl EventHandler for Chad {
    fn on_event(&mut self, cli: &mut RtmClient, event: Result<Event, Error>, json_str: &str) {
        match event {
            Ok(event) => match event {
                Event::Message(message) => {
                    match message {
                        Message::Standard {channel, text, ..} => {
                            // when are these None?
                            // can we unwrap these?
                            match (channel, text) {
                                (Some(channel), Some(text)) => {
                                    self.handle_message(cli, &channel, &text);
                                },
                                _ => ()
                            }
                        },
                        _ => (),
                    }
                },
                _ => (),
            }, 
            _ => (),
        }
        
    }

    fn on_ping(&mut self, _: &mut RtmClient) {}

    fn on_close(&mut self, _: &mut RtmClient) {}

    fn on_connect(&mut self, _: &mut RtmClient) {}
}
