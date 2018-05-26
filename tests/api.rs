extern crate actix_web;
extern crate catslapp;
extern crate failure;
extern crate futures;
extern crate serde_json;

use actix_web::test::TestServer;
use futures::prelude::*;

fn get_server() -> TestServer {
    actix_web::test::TestServer::with_factory(catslapp::app_factory)
}

#[test]
fn hello_works() {
    let mut server = get_server();

    let req = server
        .get()
        .uri(&server.url("/hello/george"))
        .finish()
        .expect("request built");

    println!("{:?}", req);

    let res = server.execute(req.send()).expect("response received");

    println!("{:?}", res);

    assert!(res.status().is_success());

    let (bytes, _) = server.execute(res.into_future()).unwrap();
    assert_eq!(
        ::std::str::from_utf8(&bytes.unwrap()).unwrap(),
        "o, hai george"
    );
}

#[test]
fn images_returns_an_array_of_strings() {
    let mut server = get_server();
    let req = server.get().uri(server.url("/cat/slaps")).finish().unwrap();

    let res = server.execute(req.send()).expect("response received");
    let (bytes, res) = server.execute(res.into_future()).unwrap();

    assert!(res.status().is_success(), res.status().to_string());
    let bytes = bytes.unwrap();
    let _parsed_body: Vec<&str> = serde_json::from_slice(&bytes).expect("is an array of strings");
}
