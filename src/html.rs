//! Handlebars templates and associated response structs
use std::str::from_utf8;
use handlebars::{Handlebars, TemplateError};
use rust_embed::RustEmbed;
use tower_web::view::Handlebars as HandlebarsSerializer;

#[derive(RustEmbed)]
#[folder = "templates"]
struct Assets;

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

pub fn init_handlebars() -> Result<HandlebarsSerializer, TemplateError> {
    let mut hb = Handlebars::new();

    hb.register_template_string("page", from_utf8(&Assets::get("page.hbs").unwrap()).expect("UTF8"))?;
    hb.register_template_string("message", from_utf8(&Assets::get("message.hbs").unwrap()).expect("UTF8"))?;

    Ok(HandlebarsSerializer::new_with_registry(hb))
}
