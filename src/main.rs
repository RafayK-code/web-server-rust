use std:: {
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    /*
     * Bind works like new. Returns a new TcpListener instance. Called bind since
     * connecting to a port to listen is known as "Binding to a port"
     */
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {

        let stream = stream.unwrap();
        
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "html/hello.html")  // If the broswer requests the right thing (127.0.0.1:7878)
    }
    else {
        ("HTTP/1.1 404 NOT FOUND", "html/404.html")  // Invalid requests (127.0.0.1:7878/foo)
    };

    let contents = fs::read_to_string(filename).unwrap(); //Read the HTML file as a string
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();

}