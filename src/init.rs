use std::{fs, io, path::{Path, PathBuf}};
use toml::Table;
use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command(name = "womscp-server")]
#[command(version = "1.0")]
#[command(about = "Server that handles the WOMSCP.", long_about = None)]
pub struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Commands>
}

#[derive(Subcommand)]
pub enum Commands {
    /// initializes the server
    Init
}


static DEFAULT_CONFIG :&'static str = "config.toml";

pub struct ServerConfig {
    pub address :String,
    pub database :String,
    pub sensor_types :Vec<String>,
    pub microcontroller_count :u16,
    pub sensors_per_microcontroller :u8
}


impl ServerConfig {
    pub fn new() -> Self {
        // NOTE: Default values for server config.
        let server_config = ServerConfig {
            address: "127.0.0.1:3000".to_string(),
            database: "sqlite:w_orchid.db".to_string(),
            sensor_types: vec![],
            microcontroller_count: 1,
            sensors_per_microcontroller: 2
        };


        if !Path::new(DEFAULT_CONFIG).exists() {
            server_config
        } else {
            DEFAULT_CONFIG.try_into().unwrap()
        }
    }
}


impl TryFrom<&str> for ServerConfig {
    type Error = io::Error;

    fn try_from(file: &str) -> Result<Self, Self::Error> {
        PathBuf::from(file).try_into()
    }
}

impl TryFrom<PathBuf> for ServerConfig {
    type Error = io::Error;

    fn try_from(file: PathBuf) -> Result<Self, Self::Error> {
        // NOTE: Default values for server config.
        let mut server_config = ServerConfig {
            address: "127.0.0.1:3000".to_string(),
            database: "sqlite:w_orchid.db".to_string(),
            sensor_types: vec![],
            microcontroller_count: 1,
            sensors_per_microcontroller: 2
        };

        let contents = fs::read_to_string(file)?;
        let mut config = match contents.parse::<Table>() {
            Ok(_config) => _config,
            Err(e) => {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, e.message()))
            }
        };

        server_config.address = if let Some(_address) = config["address"].as_str() {
            String::from(_address)
        } else {
            server_config.address
        };

        server_config.database = if let Some(_database) = config["database"].as_str() {
            String::from(_database)
        } else {
            server_config.database
        };

        if let Some(sensor_types) = config["sensor_types"].as_array_mut() {
            sensor_types.iter_mut().for_each(|t| {
                server_config.sensor_types.push(t.as_str().unwrap().to_string())
            });
        } else {
            panic!("Config error: no sensor types were provided!");
        }

        server_config.microcontroller_count = if let Some(_count) = config["microcontroller_count"].as_integer(){
            _count as u16
        } else {
            server_config.microcontroller_count
        };

        server_config.sensors_per_microcontroller = if let Some(_count) = 
            config["sensors_per_microcontroller"].as_integer() {

                _count as u8
            } else {
                server_config.sensors_per_microcontroller
            };

        Ok(server_config)
    }
}


pub async fn server_init(server_config :&ServerConfig) {
    let conn = sqlx::SqlitePool::connect(&server_config.database).await.unwrap();

    let sensor_types_it = server_config.sensor_types.iter();
    for t in sensor_types_it {
        sqlx::query(
            "INSERT INTO SensorTypes VALUES(NULL, $1)"
        )
            .bind(t)
            .execute(&conn)
            .await
            .unwrap();
    }

    for m_id in 0..server_config.microcontroller_count {
        sqlx::query(
            "INSERT INTO Microcontrollers VALUES(NULL)"
        )
            .execute(&conn)
            .await
            .unwrap();

        for s_id in 0..server_config.sensors_per_microcontroller {
            sqlx::query(
                "INSERT INTO Sensors VALUES(NULL, $1, $2)"
            )
                .bind(s_id)
                .bind(m_id)
                .execute(&conn)
                .await
                .unwrap();

            }
    }

    conn.close().await;
}
