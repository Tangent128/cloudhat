#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate tower_web;

use std::sync::Mutex;

use clap::{App, Arg};
use diesel::prelude::*;
use tower_web::ServiceBuilder;

mod schema;

use self::schema::player;

embed_migrations!();

struct CloudHatWeb {
    /// Due to low anticipated load, just keep a single database connection
    /// instead of setting up an r2d2 connection pool for now
    conn: Mutex<SqliteConnection>
}

#[derive(Queryable)]
struct Player {
    id: i32,
    #[column_name = "urlKey"]
    url_key: String,
    name: String
}

impl_web! {
    impl CloudHatWeb {
        #[get("/player/:key")]
        fn show_player(&self, key: String) -> QueryResult<String> {
            let player: QueryResult<Option<Player>> = player::table
                .filter(player::urlKey.eq(key))
                .get_result(&*self.conn.lock().unwrap()).optional();
            player.map(|option| match option {
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

    let conn = SqliteConnection::establish(args.value_of("database").unwrap()).expect("Opening Sqlite");
    let listen_to = args.value_of("address").unwrap().parse().expect("Listen address");

    // Initialize database
    embedded_migrations::run_with_output(&conn, &mut std::io::stdout()).expect("Running migrations");

    // Launch server
    ServiceBuilder::new()
        .resource(CloudHatWeb {
            conn: Mutex::new(conn)
        })
        .run(&listen_to)
        .unwrap();
}
