use tokio::net::TcpListener;

mod connections;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();

    loop {
        if let Ok((stream, _)) = listener.accept().await {
            connections::handle_connection(stream);
        }
    }
}
