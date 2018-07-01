extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio;

use futures::{Future, Stream};
use hyper::{Body, Client};
use hyper_tls::HttpsConnector;
use std::io::Write;

fn main() {
    let https = HttpsConnector::new(2).unwrap();
    let client: Client<_, Body> = Client::builder().build(https);
    let mut runtime = tokio::executor::current_thread::CurrentThread::new();
    let future = client.get("https://bash.im/".parse().unwrap())
        .and_then(|res| {
        println!("Response: {}", res.status());
        println!("Headers: {:#?}", res.headers());
        res.into_body().for_each(|chunk| {
            std::io::stdout().write_all(&chunk)
                .map_err(|e| panic!("example expects stdout is open, error={}", e))
        })
    });
    runtime.block_on(future).unwrap();
}
