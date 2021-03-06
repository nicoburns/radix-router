extern crate futures;
extern crate hyper;
extern crate pretty_env_logger;
extern crate radix_router;

use futures::future;
use hyper::rt::{self, Future};
use hyper::{Body, Request, Response, Server};
use radix_router::router::{BoxFut, Params, Router, Handler};

fn index(_: Request<Body>, _: Params) -> BoxFut {
    let res = Response::builder().body("welcome!\n".into()).unwrap();
    Box::new(future::ok(res))
}

fn hello(_: Request<Body>, ps: Params) -> BoxFut {
    // let name = ps.by_name("name").unwrap();
    let name = &ps[0];
    let res = Response::builder()
        .body(format!("hello, {}!\n", name).into())
        .unwrap();
    Box::new(future::ok(res))
}

fn main() {
    pretty_env_logger::init();

    let addr = ([127, 0, 0, 1], 3000).into();

    // new_service is run for each connection, creating a 'service'
    // to handle requests for that specific connection.
    let new_service = move || {
        // This is the `Service` that will handle the connection.
        let mut router: Router<Handler> = Router::new();
        router.get("/", Box::new(index));
        router.get("/hello/:name", Box::new(hello));
        router
    };

    let server = Server::bind(&addr)
        .serve(new_service)
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);

    rt::run(server);
}
