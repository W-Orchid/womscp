use std::net::TcpListener;
use std::io::Read;

use womscp::womscp::WOMSCP;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();

    for stream_res in listener.incoming() {
        match stream_res {
            Ok(mut stream) => {
                let mut buf :[u8; 10] = [0; 10];

                if let Err(e) = stream.read(&mut buf) {
                    eprintln!("Could not read from stream error: {:?}", e);
                } else {
                    let womscp = WOMSCP::from(buf);
                    println!("WOMSCP result: {:?}", womscp);
                }
            },
            Err(e) => {
                eprintln!("TCP stream error: {:?}", e);
            }
        } 
    }
}
