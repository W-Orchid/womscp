use tokio::net::TcpStream;

use womscp_lib::womscp::{Request, ResponseError};


pub async fn handle_connection(stream :&TcpStream) -> Result<(), ResponseError> {
    match Request::try_from(stream) {
        Ok(req) => { 
            dbg!(req);
            Ok(())
        },
        Err(res_err) => {
            eprintln!("WOMSCP parsing error: {:?}", res_err);
            return Err(res_err);
        },
    }
}
