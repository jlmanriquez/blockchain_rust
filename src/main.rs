use crate::blockchain::Blockchain;
use clap::{App, Arg};
use iron::{Iron, Request, Response, status};
use iron::mime::Mime;
use router::Router;
use std::sync::{Arc, RwLock};
use serde::{Serialize, Deserialize};

mod block;
mod blockchain;

#[derive(Serialize, Deserialize, Debug)]
struct BlockchainResponse {
    status: String,
}

fn main() {
    let matches = App::new("Blockhain Example")
        .version("1.0")
        .author("jlmanriquez")
        .about("A blockchain example taking as example an existing implementation in C++")
        .arg(Arg::with_name("difficulty")
            .short("d")
            .long("difficulty")
            .value_name("difficulty")
            .help("Difficulty use for mine block. Default value is 3")
            .required(false)
            .takes_value(true))
        .get_matches();

    let difficulty = matches.value_of("difficulty")
        .unwrap_or("3")
        .parse::<u32>()
        .unwrap();

    let repo = Arc::new(RwLock::new(Blockchain::new(difficulty)));
    let mut router = Router::new();

    let blockchain = Arc::clone(&repo);

    router.post("/block/:data", move |req: &mut Request| {
        let data = req.extensions.get::<Router>().unwrap().find("data").unwrap_or("");

        let mut blockchain = blockchain.write().unwrap();
        blockchain.add_block(data);

        Ok(Response::with(
            ("application/json".parse::<Mime>().unwrap(),
             status::Ok,
             serde_json::to_string(&BlockchainResponse {
                 status: String::from("Ok"),
             }).unwrap())))
    }, "addBlock");

    let blockchain = Arc::clone(&repo);

    router.get("/block", move |_: &mut Request| {
        let blockchain = blockchain.read().unwrap();

        Ok(Response::with(
            ("application/json".parse::<Mime>().unwrap(),
             status::Ok,
             serde_json::to_string(blockchain.get_block()).unwrap())))
    }, "showBlockchain");

    Iron::new(router).http("localhost:3000").unwrap();
}
