use std::{
    fs, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}
};

use webserver::ThreadPool;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.excute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_req = buf_reader.lines().next().unwrap().unwrap();

    let http_split: Vec<_> = http_req.split(' ').collect();

    let (status_line, filename) = if http_split[1] == "/" {
        ("HTTP/1.1 200 OK", "test.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContents-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
