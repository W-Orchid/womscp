use tokio::net::TcpListener;

use clap::Parser;

mod init;
mod connections;

#[tokio::main]
async fn main() {
    // parse CLI options and generate config
    let cli = init::Cli::parse();

    let server_config : init::ServerConfig = if let Some(path) = cli.config {
        path.try_into().unwrap()
    } else {
        init::ServerConfig::new()
    };

    if let Some(init::Commands::Init) = cli.command {
        init::server_init(&server_config).await;
    };


    // listen for oncoming connections and connect to database
    let listener = TcpListener::bind(&server_config.address).await.unwrap();

    let conn = sqlx::pool::PoolOptions::new()
        .max_connections(100)
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect(&server_config.database)
        .await.unwrap();


    loop {
        if let Ok((stream, _)) = listener.accept().await {
            connections::handle_connection(&conn, stream).await;
        }
    }
}
