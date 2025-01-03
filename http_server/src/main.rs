use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::thread;

#[derive(Debug)]
struct Request {
    method: String,
    path: String,
}

impl Request {
    fn from_stream(stream: &mut TcpStream) -> Option<Request> {
        let mut buffer = [0; 4096];
        match stream.read(&mut buffer) {
            Ok(size) => {
                let request = String::from_utf8_lossy(&buffer[..size]).to_string();
                let mut lines = request.lines();

                // Parse request line
                let request_line = lines.next()?;
                let mut parts = request_line.split_whitespace();
                let method = parts.next()?.to_string();
                let path = parts.next()?.to_string();

                // Parse headers
                let mut headers = HashMap::new();
                for line in lines {
                    if line.is_empty() {
                        break;
                    }
                    if let Some((key, value)) = line.split_once(": ") {
                        headers.insert(key.to_string(), value.to_string());
                    }
                }

                Some(Request { method, path })
            }
            Err(_) => None,
        }
    }
}

struct Response {
    status_code: u16,
    status_text: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl Response {
    fn new(status_code: u16, status_text: &str, body: Vec<u8>) -> Response {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/plain".to_string());
        headers.insert("Content-Length".to_string(), body.len().to_string());

        Response {
            status_code,
            status_text: status_text.to_string(),
            headers,
            body,
        }
    }

    fn send(&self, stream: &mut TcpStream) {
        let status_line = format!("HTTP/1.1 {} {}\r\n", self.status_code, self.status_text);
        stream.write(status_line.as_bytes()).unwrap();

        for (key, value) in &self.headers {
            let header_line = format!("{}: {}\r\n", key, value);
            stream.write(header_line.as_bytes()).unwrap();
        }

        stream.write(b"\r\n").unwrap();
        stream.write(&self.body).unwrap();
        stream.flush().unwrap();
    }
}

fn handle_static_file(path: &str) -> Response {
    let file_path = format!("static{}", path);
    match fs::read(&file_path) {
        Ok(content) => {
            let mut response = Response::new(200, "OK", content);
            let content_type = match Path::new(&file_path).extension().and_then(|s| s.to_str()) {
                Some("html") => "text/html",
                Some("css") => "text/css",
                Some("js") => "application/javascript",
                Some("png") => "image/png",
                Some("jpg") | Some("jpeg") => "image/jpeg",
                _ => "application/octet-stream",
            };
            response
                .headers
                .insert("Content-Type".to_string(), content_type.to_string());
            response
        }
        Err(_) => Response::new(404, "Not Found", b"404 - File not found".to_vec()),
    }
}

fn handle_client(mut stream: TcpStream) {
    if let Some(request) = Request::from_stream(&mut stream) {
        let response = match (request.method.as_str(), request.path.as_str()) {
            ("GET", "/") => Response::new(200, "OK", b"Welcome to Rust HTTP Server! ".to_vec()),
            ("GET", "/api/time") => {
                let time = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
                    .to_string();
                Response::new(200, "OK", time.as_bytes().to_vec())
            }
            ("GET", path) if path.starts_with("/static/") => handle_static_file(&path[7..]),
            _ => Response::new(404, "Not Found", b"404 - Page not found ".to_vec()),
        };

        response.send(&mut stream);
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}
