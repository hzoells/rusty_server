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
        handle_request(stream,root);
    }

}

fn handle_request(mut stream: TcpStream,root: &str){
    let mut request_buffer = [0;1024];
    let mut response_buffer = [0;1024];
    stream.read(&mut request_buffer).unwrap();
    let mut path = String::from(root);
    let mut status = parse_request(&request_buffer,&mut path);
    let mut resource = match status {
        RequestStatus::Pending => {
            match fs::File::open(&path) { 
                Err(why) => {
                    status = RequestStatus::NotFound;
                    fs::File::open("src/error_pages/404.html").unwrap()
                },
                Ok(file) => {
                    if fs::metadata(&path).unwrap().is_dir() {
                        status =RequestStatus::BadRequest;            
                        fs::File::open(format!("src/error_pages/{}.html",get_code(&status))).unwrap()
                    }else{
                        file
                    }
                },
            }  
        },
        _ => {
            println!("{}",format!("src/error_pages/{}.html",get_code(&status)));
            fs::File::open(format!("src/error_pages/{}.html",get_code(&status))).unwrap()
        }
    };
    send_status_line(&mut stream, &status);
    let buf_size = 1024;
    let mut bytes_read = buf_size;
    while bytes_read == buf_size {
        bytes_read = resource.read(&mut response_buffer).unwrap();
        stream.write(&mut response_buffer).unwrap();
        println!("{}",bytes_read);
    }
    stream.flush().unwrap();
}




fn parse_request (request_buffer:&[u8],path: &mut String) -> RequestStatus {
    let request = String::from_utf8_lossy(&request_buffer[..]);//needed so that we have an owner
    let request_parsed:Vec<&str> = request.split(|c| c==' ' || c=='\r' || c == '\n').collect();
    if request_parsed.len()<3 {
        RequestStatus::NotImplemented
    }else{
        let method = request_parsed[0];
        let request_uri = request_parsed[1];
        let http_version = request_parsed[2];
        if method != "GET" || (http_version != "HTTP/1.0" && http_version!="HTTP/1.1"){
            RequestStatus::NotImplemented
        }else if request_uri.find('/').unwrap_or_else(|| 1) !=0 || request_uri.contains("..") {

            RequestStatus::BadRequest
        }else{//we don't have a format error on our header and can procede to check file
            path.push_str(request_uri);
            RequestStatus::Pending
        }
    }
}

enum RequestStatus {
    OK,
    Created,
    Accepted,
    NoContent,
    MovedPerm,
    MovedTemp,
    NotModified,
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    InternalError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    Pending,
}
fn get_code(status: &RequestStatus)->u32{
    match status {
        RequestStatus::OK => 200,
        RequestStatus:: Created => 201,
        RequestStatus:: Accepted => 202,
        RequestStatus:: NoContent => 204,
        RequestStatus::MovedPerm => 301,
        RequestStatus::MovedTemp => 302,
        RequestStatus::NotModified =>304,
        RequestStatus::BadRequest => 400,
        RequestStatus::Unauthorized=>401,
        RequestStatus::Forbidden=>403,
        RequestStatus::NotFound=>404,
        RequestStatus::InternalError=>500,
        RequestStatus::NotImplemented=>501,
        RequestStatus::BadGateway=>502,
        RequestStatus::ServiceUnavailable=> 503,
        RequestStatus::Pending=>0,
    }
}

fn send_status_line(stream: &mut TcpStream, status: &RequestStatus, ){
    let reason_phrase = String::from(match status {
        RequestStatus::OK => "OK",
        RequestStatus:: Created => "Created",
        RequestStatus:: Accepted => "Accepted",
        RequestStatus:: NoContent => "Nso Content",
        RequestStatus::MovedPerm => "Moved Permanently",
        RequestStatus::MovedTemp => "Moved Temporarily",
        RequestStatus::NotModified =>"Not Modified",
        RequestStatus::BadRequest => "Bad Request",
        RequestStatus::Unauthorized=>"Unauthorized",
        RequestStatus::Forbidden=>"Forbidden",
        RequestStatus::NotFound=>"Not Found",
        RequestStatus::InternalError=>"Internal Error",
        RequestStatus::NotImplemented=>"Not Implemented",
        RequestStatus::BadGateway=>"Bad Gateway",
        RequestStatus::ServiceUnavailable=> "Service Unavailable",
        RequestStatus::Pending=>"",
    });
    let code = get_code(&status);
    stream.write(format!("HTTP/1.1 {} {}\r\n\r\n",code,reason_phrase).as_bytes()).unwrap();
}



fn die(msg: &str){
    eprintln!("{}",msg);
    process::exit(1);
}
