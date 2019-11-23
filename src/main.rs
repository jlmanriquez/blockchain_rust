use crate::blockchain::Blockchain;
use clap::{App, Arg};
use router::Router;
use std::sync::{Arc, RwLock};
use serde::{Serialize, Deserialize};
use crate::tcpserver::TCPServer;
use std::io::{Read, Write};
use std::net::Shutdown;
use std::{str};

mod block;
mod blockchain;
mod tcpserver;

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
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .value_name("port")
            .help("Server port")
            .required(false)
            .takes_value(true))
        .get_matches();

    let difficulty = matches.value_of("difficulty")
        .unwrap_or("3")
        .parse::<u32>()
        .unwrap();

    let port = matches.value_of("port")
        .unwrap_or("8090");

    let repo = RwLock::new(Blockchain::new(difficulty));
    let server = &mut tcpserver::TCPServer::new(port);

    server
        .on_incoming(move |mut s| {
            let mut data = [0 as u8; 50];

            while match s.read(&mut data) {
                Ok(size) => {
                    // Modificar logica para concatenar data de entrada
                    // y solo cuando se lea completamente la entrada
                    // se genere el nuevo bloque
                    if size < data.len() {
                        let mut blockchain = repo.write().unwrap();
                        blockchain.add_block(str::from_utf8(&data).unwrap());

                        let last_block = blockchain.get_block().last().unwrap();
                        let json = format!("{:?}", last_block);

                        s.write(json.as_bytes()).unwrap();
                        false
                    } else {
                        true
                    }
                },
                Err(_) => {
                    println!("An error occurred, terminating connection with {}", s.peer_addr().unwrap());
                    s.shutdown(Shutdown::Both).unwrap();
                    false
                }
            } {}
        })
        .on_error(move |err| {})
        .run();
}
