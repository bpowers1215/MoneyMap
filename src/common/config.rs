// src/common/config.rs

/// API Configuration

//Import Crates
extern crate toml;

//Import Modules
use std::io;
use std::io::prelude::*;
use std::fs::File;

pub struct Config {
    pub database: Database
}

#[derive(Debug)]
pub struct Database {
    pub host: Option<String>,
    pub port: Option<i64>,
    pub name: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl Config{

    /// Create A Config struct
    ///
    /// # Returns
    /// `Config` - Config struct
    pub fn new() -> Config{

        let mut database = None;

        match read_config_from_file("config/config.toml"){
            Ok(config_string) => {
                //debug!("Config String: {}", configString);
                match toml::Parser::new(&config_string).parse(){
                    Some(config_table) => {
                        database = parse_database_config(config_table);
                    }
                    None => {}
                }
            },
            Err(e) => {
                warn!("{}", e);
            }
        }

        Config {
            database: match database{
                None => Database{
                    host: None,
                    port: None,
                    name: None,
                    username: None,
                    password: None
                },
                Some(d) => d
            }
        }
    }
}

/// Read Configuration From File
///
/// # Arguments
/// * `file` - &str The path to configuration file
///
/// # Returns
/// `Result<String>` - File Contents
fn read_config_from_file(file: &'static str) -> Result<String, io::Error>{
    //open the file
    let mut f = try!(File::open(file));

    // read file contents into String
    let mut buffer = String::new();
    try!(f.read_to_string(&mut buffer));

    Ok(buffer)
}

/// Parse database configuration
///
/// # Arguments
/// * `config_table` - toml::Table The toml file represented as a BTreeMap
///
/// # Returns
/// `Option<Database>` - Database Config
fn parse_database_config(config_table: toml::Table) -> Option<Database>{
    match config_table.get("database"){
        Some(dc) => {
            match dc.as_table(){
                Some(database_config) => {
                    //std::collections::BTreeMap
                    let host = match database_config.get("host"){
                        Some(v) => {
                            //v: toml::Value
                            match v.as_str(){
                                Some(vs) => Some(vs.to_string()),
                                None => {
                                    warn!("Cannot read database host as string");
                                    None
                                }
                            }
                        },
                        None => {
                            warn!("Database host not found in configuration");
                            None
                        }
                    };
                    let port = match database_config.get("port"){
                        Some(v) => v.as_integer(),//v: toml::Value
                        None => {
                            warn!("Database port not found in configuration");
                            None
                        }
                    };
                    let name = match database_config.get("name"){
                        Some(v) => {
                            //v: toml::Value
                            match v.as_str(){
                                Some(vs) => Some(vs.to_string()),
                                None => {
                                    warn!("Cannot read database name as string");
                                    None
                                }
                            }
                        },
                        None => {
                            warn!("Database name not found in configuration");
                            None
                        }
                    };
                    let username = match database_config.get("username"){
                        Some(v) => {
                            //v: toml::Value
                            match v.as_str(){
                                Some(vs) => Some(vs.to_string()),
                                None => {
                                    warn!("Cannot read database username as string");
                                    None
                                }
                            }
                        },
                        None => {
                            warn!("Database username not found in configuration");
                            None
                        }
                    };
                    let password = match database_config.get("password"){
                        Some(v) => {
                            //v: toml::Value
                            match v.as_str(){
                                Some(vs) => Some(vs.to_string()),
                                None => {
                                    warn!("Cannot read database password as string");
                                    None
                                }
                            }
                        },
                        None => {
                            warn!("Database password not found in configuration");
                            None
                        }
                    };

                    Some(Database{
                        host: host,
                        port:  port,
                        name:  name,
                        username:  username,
                        password:  password
                    })
                },
                None => None
            }
        },
        None => {
            warn!("No `database` configuration found.");
            None
        }
    }
}
