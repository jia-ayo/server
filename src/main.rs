fn main() {

    let server =  Server::new("127.0.0.1:8080".to_string());
    server.run()
}

struct Server {
    attr: String
}

impl Server {
    fn new(attr: String) -> Self {
        Self {
            attr 
        }
    }

    fn run(self){
         println!("listening on port {}", self.attr);
    }
}