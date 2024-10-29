use tokio::net::TcpStream;

use womscp_lib::womscp::{self, ResponseError};

pub async fn handle_connection(stream :TcpStream) -> Result<(), ResponseError> {

    match womscp::Request::try_from(&stream) {
        Ok(req) => { 
            dbg!(req);
        },
        Err(res_err) => {
            eprintln!("WOMSCP parsing error: {:?}", res_err);
            return Err(res_err);
        },
    }
}
