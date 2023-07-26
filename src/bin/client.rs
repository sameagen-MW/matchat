use std::env;
use std::net::TcpStream;
use std::io::prelude::*;
use matchat::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    // Create socket
    let mut stream = TcpStream::connect(args[1].clone()).unwrap();
    println!("{:?}", stream);

    let mut last_valid = 0;
    
    // Loop HTTP request
    loop {
        let resp = reqwest::blocking::get(args[2].clone() + &last_valid.to_string());

        println!("{:?}", resp);

        match resp {
            Ok(r) => { 
                let messages = r.json::<Vec<Message>>().unwrap();
                println!("{:?}", messages);
                
                // Update max
                if messages.len() > 0 {
                    last_valid = messages[messages.len()-1].id;
                }

                stream.write(serde_json::to_string(&messages).unwrap().as_bytes()).unwrap(); 
            },
            Err(_) => (),
        }
    }
}
