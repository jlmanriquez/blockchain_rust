use crate::blockchain::Blockchain;
use clap::{App, Arg};
use iron::{Iron, Request, Response, status};
use iron::mime::{Mime};
use router::{Router};
use std::sync::{Arc, Mutex};

mod block;
mod blockchain;

fn main() {
    let matches = App::new("Blockchain Example")
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

    let repo = Arc::new(Mutex::new(Blockchain::new(difficulty)));
    let mut router = Router::new();

    router.post("/block/:data", move |req: &mut Request| {
        let repo_cloned = repo.clone();
        let data = req.extensions.get::<Router>().unwrap().find("data").unwrap_or("");

        let mut blockchain = repo_cloned.lock().unwrap();
        blockchain.add_block(data);

        let content_type = "application/json".parse::<Mime>().unwrap();

        Ok(Response::with(
            (content_type,
             status::Ok,
             serde_json::to_string(blockchain.get_last()).unwrap())))
    }, "addBlock");

    Iron::new(router).http("localhost:3000").unwrap();
}
