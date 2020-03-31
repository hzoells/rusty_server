use std::fs;
use std::env;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::process;
mod request;

fn main() {
    /*
     * Parse command line arguments
     * */
    let args: Vec<String> = env::args().collect();
    if args.len()!=3{
        die("Usage: ./rusty_server <port> <root>");
    }
    let root = &args[1];
    let port = &args[2];
    /*
     * Set up listener
     * */
    let listener = TcpListener::bind(format!("127.0.0.1:{}",port)).unwrap();
    for stream in listener.incoming() {
        println!("We got a stream");
        let stream = stream.unwrap();
        handle_request(stream,root);
    }

}

fn handle_request(mut stream: TcpStream,root: &str){
    let mut request = request::Request::new(stream, root);
    request.parse_request();
    request.respond();
}


fn die(msg: &str){
    eprintln!("{}",msg);
    process::exit(1);
}
