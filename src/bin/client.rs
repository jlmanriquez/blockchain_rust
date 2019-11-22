use std::net::{TcpStream, Shutdown};
use std::io::{Write, Read};
use std::str::from_utf8;
use std::thread;

fn main() {
    let mut all_threads = vec![];

    for id in 0..2 {
        all_threads.push(thread::spawn(move || start_client(id)));
    }

    for t in all_threads {
        t.join();
    }

    println!("Terminated.");
}

fn start_client(id: i32) {
    match TcpStream::connect("localhost:8090") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 8090");

            let mut msg = id.to_string().to_owned();
            msg.push_str("Hello!");

            stream.write(msg.as_bytes()).unwrap();
            stream.flush().unwrap();
            println!("Sent Hello, awaiting reply...");
            let mut data = [0 as u8; 7];
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    let text = from_utf8(&data).unwrap();
                    println!("Reply: {}", text);
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}