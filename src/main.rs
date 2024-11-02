use tokio::net::TcpListener;
use sqlx::sqlite::SqlitePool;
use clap::Parser;

mod init;
mod connections;

#[tokio::main]
async fn main() {

    let cli = init::Cli::parse();

    let server_config : init::ServerConfig = match &cli.command {
        Some(init::Commands::Init { config }) => {
            todo!("init server");
        },
        None => { init::ServerConfig::new() }
    };

    let listener = TcpListener::bind(&server_config.address).await.unwrap();
    let conn = SqlitePool::connect(&server_config.database).await.unwrap();

    dbg!(&server_config.address);

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
