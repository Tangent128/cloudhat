#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate tower_web;

use clap::{App, Arg};
use diesel::prelude::*;
use tower_web::ServiceBuilder;

mod database;
mod schema;

use crate::database::{Database};

struct CloudHatWeb {
    db: Database
}

impl_web! {
    impl CloudHatWeb {
        #[get("/player/:key")]
        fn show_player(&self, key: String) -> QueryResult<String> {
            self.db.player_from_key(&key).map(|option| match option {
                Some(player) => format!("Player's name is {}", player.name),
                None => "Player does not exist, this should really be a 404 silly".into()
            })
        }
    }
}

fn main() {
    // Parse arguments
    let usage = App::new("Cloud Hat")
        .arg(Arg::from_usage("<database> 'The database file to keep state in (will be created if absent)'"))
        .arg(Arg::from_usage("<address> 'The address & port to listen on'"));

    let args = usage.get_matches();

    let listen_to = args.value_of("address").unwrap().parse().expect("Listen address");

    // Launch server
    ServiceBuilder::new()
        .resource(CloudHatWeb {
            db: Database::connect(args.value_of("database").unwrap())
        })
        .run(&listen_to)
        .unwrap();
}
