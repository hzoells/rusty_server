use std::fs;
use std::env;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::process;
use std::sync::Arc;

mod request;
mod thread_pool;

fn main() {
    /*
     * Parse command line arguments
     * */
    let args: Vec<String> = env::args().collect();
    if args.len()!=4{
        die("Usage: ./rusty_server <root> <port> <threads>");
    }
    let port = &args[2];
    let threads = args[3].parse::<usize>().expect("Invalid thread count");
    /*
     * Set up listener
     * */
    let listener = TcpListener::bind(format!("127.0.0.1:{}",port)).unwrap();
    let pool = thread_pool::ThreadPool::new(threads);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let root = String::clone(&args[1]);
        pool.execute(move || {handle_request(stream,root)})
    }
}

fn handle_request(mut stream: TcpStream,root: String){
    let mut request = request::Request::new(stream, &root);
    request.parse_request();
    request.respond();
}


fn die(msg: &str){
    eprintln!("{}",msg);
    process::exit(1);
}
