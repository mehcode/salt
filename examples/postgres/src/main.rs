extern crate futures_state_stream;
extern crate pretty_env_logger;
#[allow(unused_extern_crates)]
extern crate serde;
extern crate serde_json;
extern crate shio;
extern crate tokio_postgres as postgres;
extern crate uuid;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate error_chain;

// TODO: Connection pooling

mod errors {
    error_chain! {
        foreign_links {
            PostgresConnect(::postgres::error::ConnectError);
            Postgres(::postgres::error::Error);
            Json(::serde_json::Error);
        }
    }
}

use futures_state_stream::StateStream;
use postgres::{Connection, TlsMode};
use shio::prelude::*;
use errors::*;

#[derive(Serialize)]
struct Person {
    id: uuid::Uuid,
    name: String,
}

const DATABASE_URL: &'static str = "postgres://postgres@localhost/shio_dev_example";

fn index(ctx: Context) -> BoxFuture<Response, Error> {
    Connection::connect(DATABASE_URL, TlsMode::None, &ctx.handle())
        .from_err::<Error>()
        .and_then(|conn| {
            conn.batch_execute(
                r#"
                CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
                CREATE TABLE IF NOT EXISTS person (
                    id UUID PRIMARY KEY DEFAULT uuid_generate_v1mc(),
                    name TEXT NOT NULL
                );
            "#,
            ).from_err()
        })
        .and_then(|conn| {
            conn.prepare("INSERT INTO person (name) VALUES ($1)")
                .from_err()
        })
        .and_then(move |(stmt, conn)| {
            let name: &str = &ctx.get::<Parameters>()["name"];

            conn.execute(&stmt, &[&name]).from_err()
        })
        .and_then(|(_, conn)| {
            conn.prepare("SELECT id, name FROM person").from_err()
        })
        .and_then(|(stmt, conn)| {
            conn.query(&stmt, &[])
                .map(|row| {
                    Person {
                        id: row.get("id"),
                        name: row.get("name"),
                    }
                })
                .collect()
                .from_err()
        })
        .and_then(|(results, _)| {
            let s = serde_json::to_string(&results)?;

            Ok(
                Response::build()
                    .header(http::header::ContentType::json())
                    .body(s),
            )
        })
        .into_box()
}

fn main() {
    pretty_env_logger::init().unwrap();

    Shio::default()
        .route((Method::GET, "/{name}", index))
        .run(":7878")
        .unwrap();
}
