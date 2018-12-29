//! Handlebars templates and associated response structs
use std::ffi::OsStr;
use std::path::Path;
use std::str::from_utf8;
use handlebars::Handlebars;
use http::Response as HttpResponse;
use rust_embed::RustEmbed;
use tower_web::view::Handlebars as HandlebarsSerializer;

#[derive(RustEmbed)]
#[folder = "templates"]
struct Assets;

#[derive(Debug, PartialEq, Response)]
#[web(either)]
pub enum View {
    Message(Message)
}

#[derive(Debug, PartialEq, Response)]
#[web(template = "message")]
pub struct Message {
    #[web(status)]
    status: u16,
    text: String
}

/// A minimal view presenting an error or diagnostic message
pub fn message(code: u16, text: String) -> View {
    View::Message(Message {
        status: code,
        text
    })
}

pub fn serializer() -> HandlebarsSerializer {
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

pub struct ThemeResource;

impl_web!{
    impl ThemeResource {
        /// serve a text-based asset from the embedded assets
        #[get("/assets/:path")]
        fn serve_asset(&self, path: String) -> Result<HttpResponse<String>, http::Error> {
            let mut response = HttpResponse::builder();
            if let Some(asset_bytes) = Assets::get(&path) {
                let asset = from_utf8(&asset_bytes).expect("utf8");
                let ext = Path::new(&path).extension().and_then(|ext| ext.to_str());
                response.header("Content-Type", match ext {
                    Some("css") => "text/css",
                    _ => "text/plain"
                }).body(asset.to_string())
            } else {
                response.status(404).body("not found".into())
            }
        }
    }
}
