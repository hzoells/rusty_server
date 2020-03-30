use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;

pub struct Request {
    path: String,
    status: RequestStatus,
    stream: TcpStream,
}

impl Request {
    pub fn new(stream: TcpStream, root: &str)->Request{
        Request {
            path: String::from(root),
            status: RequestStatus::Pending,
            stream,
        }
    }
    pub fn parse_request(&mut self) -> u32{
        let mut request_buffer = [0;1024];
        self.stream.read(&mut request_buffer).unwrap();
        let request = String::from_utf8_lossy(&request_buffer[..]);//needed so that we have an owner
        let request_parsed:Vec<&str> = request.split(|c| c==' ' || c=='\r' || c == '\n').collect();
        if request_parsed.len()<3 {
            self.status = RequestStatus::NotImplemented
        }else{
            let method = request_parsed[0];
            let request_uri = request_parsed[1];
            let http_version = request_parsed[2];
            if method != "GET" || (http_version != "HTTP/1.0" && http_version!="HTTP/1.1"){
                self.status = RequestStatus::NotImplemented
            }else if request_uri.find('/').unwrap_or_else(|| 1) !=0 || request_uri.contains("..") {

                self.status = RequestStatus::BadRequest
            }else{//we don't have a format error on our header and can procede to check file
                self.path.push_str(request_uri);
                self.status = RequestStatus::Pending
            }
        }
        get_code(&self.status)
    }
    pub fn respond(&mut self){
        let mut response_buffer = [0;1024];
        let buf_size = 1024;
        let mut resource = match self.status {
            RequestStatus::Pending => {
                match fs::File::open(&(self.path)) { 
                    Err(why) => {
                        self.status = RequestStatus::NotFound;
                        fs::File::open("src/error_pages/404.html").unwrap()
                    },
                    Ok(file) => {
                        if fs::metadata(&(self.path)).unwrap().is_dir() {
                            self.status =RequestStatus::BadRequest;            
                            fs::File::open(format!("src/error_pages/{}.html",get_code(&(self.status)))).unwrap()
                        }else{
                            file
                        }
                    },
                }  
            },
            _ => {
                println!("{}",format!("src/error_pages/{}.html",get_code(&(self.status))));
                fs::File::open(format!("src/error_pages/{}.html",get_code(&(self.status)))).unwrap()
            }
        };
        self.send_status_line();
        let mut bytes_read = buf_size;
        while bytes_read == buf_size {
            bytes_read = resource.read(&mut response_buffer).unwrap();
            self.stream.write(&mut response_buffer).unwrap();
        }
        self.stream.flush().unwrap();
    }
    pub fn send_status_line(&mut self){
        let reason_phrase = String::from(match self.status {
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
        let code = get_code(&(self.status));
        self.stream.write(format!("HTTP/1.1 {} {}\r\n\r\n",code,reason_phrase).as_bytes()).unwrap();
        println!("{} {}",code,reason_phrase)
    }
}

pub enum RequestStatus {
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
pub fn get_code(status: &RequestStatus)->u32{
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