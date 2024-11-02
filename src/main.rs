use tokio::net::TcpListener;
use sqlx::sqlite::SqlitePool;

mod init;
mod connections;

#[tokio::main]
async fn main() {
    let server_address = "127.0.0.1:3000";

    let listener = TcpListener::bind(server_address).await.unwrap();
    let conn = SqlitePool::connect().await.unwrap();

    dbg!(server_address);

    loop {
        if let Ok((stream, _)) = listener.accept().await {
            let res = connections::handle_connection(&conn, &stream)
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
