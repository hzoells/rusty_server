use std::fs;
use std::env;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::process;
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
        let stream = stream.unwrap();
        handle_request(stream);
    }
}

fn handle_request(mut stream: TcpStream){
    let mut request_buffer = [0;1024];
    stream.read(&mut request_buffer).unwrap();
    let mut path = String::new();
    let status = parse_request(&request_buffer,&mut path);//consider making string method
    //make status an enum maybe
   
}




fn parse_request (request_buffer:&[u8],path: &mut String) -> u32 {
    let request = String::from_utf8_lossy(&request_buffer[..]);//needed so that we have an owner
    let request_parsed:Vec<&str> = request.split(|c| c==' ' || c=='\r' || c == '\n').collect();
    if request_parsed.len()<3 {
        501//not implemented or bad request
    }else{
        let method = request_parsed[0];
        let request_uri = request_parsed[1];
        let http_version = request_parsed[2];
        if method != "GET" || (http_version != "HTTP/1.0" && http_version!="HTTP/1.1"){
            501//not implemented
        }else if request_uri.find('/').unwrap_or_else(|| 1) !=0 || request_uri.contains("..") {
            400//bad request
        }else{//we don't have a format error on our header and can procede to check file
            path.push_str(request_uri);
            0
        }
    }
}

enum Request_status {
    OK(200),
    Created(201),
    Accepted(202),
    NoContent(204),
    MovedPerm(301),
    MovedTemp(302),
    NotModified(304),
    BadRequest(400),
    Unauthorized(401),
    Forbidden(403),
    NotFound(404),
    InternalError(500),
    NotImplemented(501),
    BadGateway(502),
    ServiceUnavailable(503),
}
fn get_code(status: &Request_status)->u32{
    match status {
        Request_status::OK => 200,
        Request_status:: Created => 201,


    }
}


fn die(msg: &str){
    eprintln!("{}",msg);
    process::exit(1);
}
