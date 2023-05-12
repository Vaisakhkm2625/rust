use std::net::TcpListener;

fn main() {
    println!("Hello, world!");

    let listerner = TcpListener::bind("127.0.0.1:8000").unwrap();

    for stream in listerner.incoming() {
        let stream = stream.unwrap();

        println!("a connection was made");
    }
}
