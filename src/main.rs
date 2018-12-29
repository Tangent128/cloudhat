#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate tower_web;

use clap::{App, Arg};
use diesel::prelude::*;

use tower_web::ServiceBuilder;

embed_migrations!();

struct CloudHatWeb {
}

impl_web! {
    impl CloudHatWeb {
        #[get("/player/:key")]
        fn show_player(&self, _key: String) -> Result<String, ()> {
            Ok("Hello, World".into())
        }
    }
}

fn main() {
    // Parse arguments
    let usage = App::new("Cloud Hat")
        .arg(Arg::from_usage("<database> 'The database file to keep state in (will be created if absent)'"))
        .arg(Arg::from_usage("<address> 'The address & port to listen on'"));

    let args = usage.get_matches();

    let conn = SqliteConnection::establish(args.value_of("database").unwrap()).expect("Opening Sqlite");
    let listen_to = args.value_of("address").unwrap().parse().expect("Listen address");

    // Initialize database
    embedded_migrations::run_with_output(&conn, &mut std::io::stdout()).expect("Running migrations");

    // Launch server
    ServiceBuilder::new()
        .resource(CloudHatWeb {
        })
        .run(&listen_to)
        .unwrap();
}
