extern crate futures;
extern crate hyper;

use hyper::{Get, StatusCode};
use hyper::header::ContentType;
use hyper::server::{Server, Service, Request, Response};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;

extern crate handlebars;
use handlebars::Handlebars;

#[derive(Clone, Copy)]
struct Echo;

impl Service for Echo {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = ::futures::Finished<Response, hyper::Error>;

    fn call(&self, req: Request) -> Self::Future {
        ::futures::finished(match (req.method(), req.path()) {
            (&Get, Some("/")) | (&Get, Some("/echo")) => {
                let handlebars = Handlebars::new();

                let mut f = File::open("templates/index.hbs").unwrap();
                let mut source = String::new();

                f.read_to_string(&mut source).unwrap();

                let mut data: BTreeMap<String, String> = BTreeMap::new();

                data.insert("foo".to_string(), "foo".to_string());

                Response::new()
                    .with_header(ContentType::html())
                    .with_body(handlebars.template_render(&source, &data).unwrap())
            },
            _ => {
                Response::new()
                    .with_status(StatusCode::NotFound)
            }
        })
    }

}


fn main() {
    let addr = "127.0.0.1:1337".parse().unwrap();
    let (listening, server) = Server::standalone(|tokio| {
        Server::http(&addr, tokio)?
            .handle(|| Ok(Echo), tokio)
    }).unwrap();
    println!("Listening on http://{}", listening);
    server.run();
}

