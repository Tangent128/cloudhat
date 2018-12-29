//! Handlebars templates and associated response structs
use std::ffi::OsStr;
use std::path::Path;
use std::str::from_utf8;
use handlebars::Handlebars;
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

pub fn init_handlebars() -> HandlebarsSerializer {
    let mut hb = Handlebars::new();

    for path_str in Assets::iter() {
        let path = Path::new(&*path_str);
        if path.extension() == Some(OsStr::new("hbs")) {
            let name = path.file_stem().unwrap().to_str().expect("utf8");
            let template_bytes = Assets::get(&path_str).unwrap();
            let template = from_utf8(&template_bytes).expect("utf8");
            hb.register_template_string(name, template).expect("Parsing template");
        }
    }

    HandlebarsSerializer::new_with_registry(hb)
}
