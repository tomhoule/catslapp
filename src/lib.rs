#![feature(crate_visibility_modifier)]
#![feature(proc_macro, generators)]

extern crate actix_web;
extern crate bytes;
extern crate failure;
extern crate futures;
// extern crate hyper;
// extern crate hyper_tls;
#[macro_use]
extern crate log;
extern crate prost;
#[macro_use]
extern crate prost_derive;
extern crate prost_types;
#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate tokio_core;
extern crate tokio_io;
extern crate uuid;

use actix_web::Json;
use futures::prelude::*;
use uuid::Uuid;

pub mod app {
    pub mod catsl {
        pub mod media {
            include!(concat!(env!("OUT_DIR"), "/app.catsl.media.rs"));
        }
    }
}

// #[async]
// pub fn add_medium(
//     url: hyper::Uri,
//     handle: tokio_core::reactor::Handle,
// ) -> Result<Uuid, failure::Error> {
//     let client = hyper::Client::configure()
//         .connector(::hyper_tls::HttpsConnector::new(4, &handle)?)
//         .build(&handle);
//     let res = await!(client.get(url))?;
//     info!("response: {:?}", res);
//     let chunks: Vec<hyper::Chunk> = await!(res.body().collect())?;
//     let mut bytes: Vec<u8> = Vec::new();
//     for chunk in chunks {
//         bytes.extend(chunk);
//     }
//     // let id = String::from_utf8(bytes)?.parse()?;
//     let id = Uuid::new_v4();
//     Ok(id)
// }

pub fn medium_created() -> app::catsl::media::MediumAdded {
    use std::default::Default;
    app::catsl::media::MediumAdded::default()
}

pub fn add_medium_proto() -> app::catsl::media::AddMedium {
    use std::default::Default;
    app::catsl::media::AddMedium::default()
}

fn hello(info: actix_web::Path<String>) -> String {
    format!("o, hai {}", info.into_inner())
}

fn cat_slaps_index(_: actix_web::Path<()>) -> actix_web::Json<serde_json::Value> {
    Json(json!([
        "https://i.redd.it/2ng5oz4dr8y01.gif",
        "https://i.redd.it/ih37hk1qo9x01.jpg",
    ]))
}

pub fn app_factory() -> actix_web::App {
    let cors_middleware = actix_web::middleware::cors::Cors::build()
        .send_wildcard()
        .finish();
    actix_web::App::new()
        .middleware(cors_middleware)
        .route("/hello/{name}", actix_web::http::Method::GET, hello)
        .resource("/cat/slaps", |r| r.get().with(cat_slaps_index))
}

pub fn start_server() -> Result<(), failure::Error> {
    actix_web::server::new(|| app_factory())
        .bind("127.0.0.1:8080")?
        .run();
    Ok(())
}
