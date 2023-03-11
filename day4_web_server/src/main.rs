use std::net::TcpListener;

/* Working on a webserver using rust programming */
// 1. Learn about tcp and http connection
// 2. Listen for TCP connections on socket
// 3. parse a small number of HTTP requests
// 4. create a proper HTTP response
// 5. improve throughput of our server with thread pool


/* A webserver is guided by two web protocols which are the request-response
protocol. The client initiates the request and the server responds */
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let _stream = stream.unwrap();

        println!("Connection established");
    }
}
