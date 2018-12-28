#[macro_use]
extern crate diesel_migrations;

use clap::{App, Arg};
use diesel::prelude::*;

embed_migrations!();

fn main() {
    let usage = App::new("Cloud Hat")
        .arg(Arg::from_usage("<database> 'The database file to keep state in (will be created if absent)'"));

    let args = usage.get_matches();

    let conn = SqliteConnection::establish(args.value_of("database").unwrap()).expect("Opening Sqlite");

    embedded_migrations::run_with_output(&conn, &mut std::io::stdout()).expect("Running migrations");
}
