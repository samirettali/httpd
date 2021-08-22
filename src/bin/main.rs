use http::ThreadPool;
use regex::Regex;
use std::fs;
use std::fs::metadata;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(10);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8(buffer.to_vec()).unwrap();

    let re = Regex::new(r"GET /(?P<path>[^\s]*) HTTP/1\.1").unwrap();

    let matches = re.captures(&request).unwrap();
    let requested_path = match &matches["path"] {
        "" | "/" => ".",
        x => x,
    };

    let status_line = "HTTP/1.1 200 OK";
    let mut contents: String = String::from("");

    let md = metadata(requested_path).unwrap();

    if md.is_dir() {
        let paths = fs::read_dir(requested_path).unwrap();

        contents = String::from("<pre>\n");

        for path in paths {
            let path = path.unwrap().path().display().to_string();
            let filename = path
                .strip_prefix(format!("{}/", requested_path).as_str())
                .unwrap();
            contents.push_str(format!("<a href='{}'>{}</a>\n", path, filename).as_str());
        }

        contents.push_str("</pre>");
    } else if md.is_file() {
        contents = fs::read_to_string(requested_path).unwrap();
    } else {
        panic!("Unknown file type: {:?}", md)
    }

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
