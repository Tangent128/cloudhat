//! Handlebars templates and associated response structs

#[derive(Debug, PartialEq, Response)]
#[web(either)]
pub enum Responses {
    Message(Message)
}

#[derive(Debug, PartialEq, Response)]
pub struct Message {
    #[web(status)]
    status: u16,
    text: String
}

impl Message {
    pub fn new(code: u16, text: String) -> Responses {
        Responses::Message(Message {
            status: code,
            text
        })
    }
}
