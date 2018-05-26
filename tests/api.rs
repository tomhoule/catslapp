extern crate actix_web;
extern crate catslapp;
extern crate failure;
extern crate futures;

use futures::prelude::*;

#[test]
fn hello_works() {
    let mut server = actix_web::test::TestServer::with_factory(catslapp::app_factory);

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
