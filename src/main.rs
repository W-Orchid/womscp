use tokio::net::TcpListener;
use sqlx::sqlite::SqlitePool;

mod connections;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    let conn = SqlitePool::connect("sqlite:w_orchid.db").await.unwrap();

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
