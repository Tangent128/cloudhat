//! Handlebars templates and associated response structs
use handlebars::Handlebars;
use tower_web::view::Handlebars as HandlebarsSerializer;

#[derive(Debug, PartialEq, Response)]
#[web(either)]
pub enum Responses {
    Message(Message)
}

#[derive(Debug, PartialEq, Response)]
#[web(template = "message")]
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

const MESSAGE_TMPL: &str = r#"
    <b>{{text}}</b>
"#;

pub fn init_handlebars() -> HandlebarsSerializer {
    let mut hb = Handlebars::new();

    hb.register_template_string("message", MESSAGE_TMPL).expect("Parsing template");

    HandlebarsSerializer::new_with_registry(hb)
}
