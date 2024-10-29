use std::net::TcpStream;
use std::io::{Write, Error};

use womscp_lib::womscp::{self, ResponseError};

pub fn handle_connection(stream_res :Result<TcpStream, Error>) {
        let response :Result<(), ResponseError>;

        match stream_res {
            Ok(mut stream) => {
                match womscp::Request::try_from(&stream) {
                    Ok(req) => { 
                        dbg!(req);
                        response = Ok(());
                    },
                    Err(res_err) => {
                        eprintln!("WOMSCP parsing error: {:?}", res_err);
                        response = Err(res_err)
                    },
                }

                if let Err(tcp_err) = match response {
                    Ok(_) => stream.write(&[0]),
                    Err(e) => stream.write(&[e as u8])
                } {
                    eprintln!("TCP write error: {:?}", tcp_err);
                }
            },

            Err(tcp_err) => {
                eprintln!("TCP stream error: {:?}", tcp_err);
            }
        } 
}
