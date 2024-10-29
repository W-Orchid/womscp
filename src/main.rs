use std::net::TcpListener;

mod connections;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    for stream_res in listener.incoming() {
        connections::handle_connection(stream_res);
    }
}
