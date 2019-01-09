#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate tower_web;

use clap::{App, Arg};
use diesel::prelude::*;
use tower_web::ServiceBuilder;

mod database;
mod theme;
mod schema;

use crate::database::{Database};
use crate::theme::{
    message,
    serializer,
    ThemeResource,
    view,
    View
};

struct CloudHatWeb {
    db: Database
}

impl_web! {
    impl CloudHatWeb {
        #[get("/player/:key")]
        #[content_type("html")]
        fn show_player(&self, key: String) -> QueryResult<View> {
            self.db.player_from_key(&key).map(|option| match option {
                Some(player) => view(|model| model.player = Some(player)),
                None => message(404, "Player does not exist".into())
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
        .resource(ThemeResource)
        .serializer(serializer())
        .run(&listen_to)
        .unwrap();
}

#[cfg(test)]
mod test {
    use super::*;

    /// make an empty database for testing
    fn test_controller() -> CloudHatWeb {
        CloudHatWeb {
            db: Database::connect(":memory:")
        }
    }

    #[test]
    fn show_player() {
        let app = test_controller();

        assert_eq!(app.show_player("nonsense".into()), Ok(message(404, "Player does not exist".into())));
    }
}
