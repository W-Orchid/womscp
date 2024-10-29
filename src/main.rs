use tokio::net::TcpListener;

mod connections;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();

    loop {
        if let Ok((stream, _)) = listener.accept().await {
            let res = connections::handle_connection(&stream)
                .await;

            if let Err(tcp_err) = match res {
                Ok(_) => stream.try_write(&[0]),
                Err(res_err) => stream.try_write(&[res_err as u8])
            } {
                eprintln!("TCP write error: {:?}", tcp_err);
            }
        }
    }
}
