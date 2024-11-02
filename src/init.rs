use std::path::Path;
use std::{fs, io};
use toml::Table;
use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command(name = "womscp-server")]
#[command(version = "1.0")]
#[command(about = "Server that handles the WOMSCP.", long_about = None)]
pub struct Cli {
#[command(subcommand)]
    pub command: Option<Commands>
}

#[derive(Subcommand)]
pub enum Commands {
    /// initializes the server
    Init {
        /// optional user-defined config file
        #[arg(short, long)]
        config: Option<String>
    }
}


static DEFAULT_CONFIG :&'static str = "config.toml";

pub struct ServerConfig {
    pub address :String,
    pub database :String,
    pub microcontroller_count :u16,
    pub sensors_per_microcontroller :u8
}


impl ServerConfig {
    pub fn new() -> Self {
        // NOTE: Default values for server config.
        let server_config = ServerConfig {
            address: "127.0.0.1:3000".to_string(),
            database: "sqlite:w_orchid.db".to_string(),
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


impl TryFrom<&'static str> for ServerConfig {
    type Error = io::Error;

    fn try_from(file: &'static str) -> Result<Self, Self::Error> {
        // NOTE: Default values for server config.
        let mut server_config = ServerConfig {
            address: "127.0.0.1:3000".to_string(),
            database: "sqlite:w_orchid.db".to_string(),
            microcontroller_count: 1,
            sensors_per_microcontroller: 2
        };

        let contents = fs::read_to_string(file)?;
        let config = match contents.parse::<Table>() {
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
