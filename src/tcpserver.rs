use std::net::{TcpListener, TcpStream};
use std::io::Error;
use std::thread;
use std::sync::Arc;

pub struct TCPServer {
    listener: TcpListener,
    incoming_fn: Option<Arc<dyn Fn(TcpStream) + Send + Sync + 'static>>,
    error_fn: Option<Box<dyn Fn(Error) + 'static>>,
}

impl TCPServer {
    pub fn new(port: &str) -> Self {
        TCPServer {
            listener: TcpListener::bind(format!("{}:{}", "0.0.0.0", port)).unwrap(),
            incoming_fn: Option::None,
            error_fn: Option::None,
        }
    }

    pub fn on_incoming<F>(&mut self, f: F) -> &mut Self
        where F: Fn(TcpStream) + Send + Sync + 'static {
        self.incoming_fn = Option::Some(Arc::new(f));
        self
    }

    pub fn on_error<F>(&mut self, f: F) -> &mut Self
        where F: Fn(Error) + 'static {
        self.error_fn = Option::Some(Box::new(f));
        self
    }

    pub fn run(&self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection: {}", stream.peer_addr().unwrap());

                    let action = Arc::clone(self.incoming_fn.as_ref().unwrap());

                    thread::spawn(move || action(stream));
                },
                Err(e) => {
                    self.error_fn.as_ref().unwrap()(e);
                }
            }
        }
    }
}