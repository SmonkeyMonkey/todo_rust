use std::env;

use actix_web::error::ErrorServiceUnavailable;
use actix_web::{Error, FromRequest};
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;
use futures::future::{err, ok, Ready};
use lazy_static::lazy_static;

type PgPool = Pool<ConnectionManager<PgConnection>>;

pub struct DbConnection {
    pub db_connection: PgPool,
}

lazy_static! {
    pub static ref DBCONNECTION: DbConnection = {
        let connection_string = env::var("DB_URL").unwrap();
        DbConnection {
            db_connection: PgPool::builder()
                .max_size(8)
                .build(ConnectionManager::new(connection_string))
                .expect("failed to create db connection pool"),
        }
    };
}

// pub fn estabilish_connection() -> PooledConnection<ConnectionManager<PgConnection>> {
//     DBCONNECTION.db_connection.get().unwrap()
// }
pub struct DB {
    pub connection: PooledConnection<ConnectionManager<PgConnection>>,
}

impl FromRequest for DB {
    type Error = Error;
    type Future = Ready<Result<DB, Error>>;

    fn from_request(_: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        match DBCONNECTION.db_connection.get() {
            Ok(connection) => ok(DB { connection }),
            Err(_) => err(ErrorServiceUnavailable(
                "could not make connection to database",
            )),
        }
    }
}
