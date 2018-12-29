//! Handlebars templates and associated response structs
use handlebars::{Handlebars, TemplateError};
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

const PAGE_TMPL: &str = r#"<html><head>
<title>{{> title}}</title>
</head><body>
{{~> content~}}
</body></html>"#;

const MESSAGE_TMPL: &str = r#"{{#> page}}
{{#*inline "title"}}{{text}}{{/inline}}
{{#*inline "content"}}
<section>
    <b>{{text}}</b>
</section>
{{/inline}}
{{/page}}"#;

pub fn init_handlebars() -> Result<HandlebarsSerializer, TemplateError> {
    let mut hb = Handlebars::new();

    hb.register_template_string("page", PAGE_TMPL)?;
    hb.register_template_string("message", MESSAGE_TMPL)?;

    Ok(HandlebarsSerializer::new_with_registry(hb))
}
