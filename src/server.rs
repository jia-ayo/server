
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

