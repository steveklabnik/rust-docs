extern crate dotenv;
extern crate futures;
extern crate sparkles;

use futures::future::BoxFuture;

use sparkles::Request;
use sparkles::Response;
use sparkles::Error;
use sparkles::ResponseBuilder;
use sparkles::Status;

use std::env;

fn main() {
    dotenv::dotenv().ok();

    let addr = format!("0.0.0.0:{}",
                       env::args().nth(1).unwrap_or(String::from("1337")))
        .parse()
        .unwrap();

    let mut server = sparkles::Server::new("templates".to_string());

    server.add_route("/", root);

    server.run(&addr);
}

fn root(_: Request) -> BoxFuture<Response, Error> {
    let mut res = ResponseBuilder::new();

    res.with_template("index".to_string());

    res.with_status(Status::Ok);

    res.to_response().into_future()
}
