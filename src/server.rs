use std::{net::TcpListener, io::Read};

pub struct Server {
    addr: String
}  

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr 
        }
    }

    pub fn run(self){
        println!("listening on port {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read( &mut  buffer) {
                        Ok(_) => {
                            println!("Recieved a request: {}", String::from_utf8_lossy(&buffer));
                        }
                        Err(e) => println!("failed to establish a connectionon: {}", e),
                    }
                } 
                Err(e) => println!("failed to establish a connection: {}", e)
            }
        }
    }
}

