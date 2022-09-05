use server::Server;
use http::request::Request;

fn main() {

    let server =  Server::new("127.0.0.1:8080".to_string());
    server.run()
}
mod server{
    pub struct Server {
        attr: String
    }
    
    impl Server {
        pub fn new(attr: String) -> Self {
            Self {
                attr 
            }
        }
    
        pub fn run(self){
             println!("listening on port {}", self.attr);
        }
   }
}

mod http {
    pub mod request{
        use super::method::Method;
        pub struct Request {
            path: String,
            query_string: Option<String>,
            method: Method,
        }
    }

    pub mod method{
        pub enum Method{
            GET,
            DELETE, 
            POST,
            PUT, 
            HEAD,
            CONNECT,
            OPTIONS,
            TRACE,
            PATCH
        }
    }
}