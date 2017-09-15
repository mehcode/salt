#[allow(unused_extern_crates)]
extern crate serde;
extern crate serde_json;
extern crate shio;

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate serde_derive;

use shio::prelude::*;

mod errors {
    error_chain! {
        foreign_links {
            Shio(::shio::Error);
            Json(::serde_json::Error);
        }
    }
}

#[derive(Debug, Deserialize)]
struct RequestBody {
    visible: bool,
    name: String,
}

#[derive(Debug, Serialize)]
struct ResponseBody {
    id: u8,
    name: String,
}

fn index(ctx: Context) -> BoxFuture<Response, errors::Error> {
    // `concat2` will asynchronously read each chunk of the request body and
    // return a single, concatenated, chunk
    ctx.data().concat2()
        // `Future::from_err` acts like `?` in that it coerces the error type from
        // the future into the final error type
        .from_err()
        // `Future::and_then` can be used to merge an asynchronous workflow with a
        // synchronous workflow
        .and_then(|data| /* -> errors::Result<Response> */ {
            let body: RequestBody = serde_json::from_slice(&data)?;
            let s = serde_json::to_string(&ResponseBody { id: 20, name: body.name })?;

            Ok(Response::build().header(http::header::ContentType::json()).body(s))
        })
        // Put our future inside a Box so we can name our return type
        // This part will go away once `impl Trait` is stablized in Rust
        .into_box()
}

fn main() {
    Shio::default()
        .route((Method::POST, "/", index))
        .run(":7878")
        .unwrap();
}
