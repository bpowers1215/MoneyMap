// src/common/config.rs

/// API Configuration

//Import Crates
extern crate toml;

//Import Modules
use std::io;
use std::io::prelude::*;
use std::fs::File;

#[derive(Clone)]
pub struct Config {
    pub database: Database,
    pub auth: Auth
}

#[derive(Clone, Debug)]
pub struct Database {
    pub host: Option<String>,
    pub port: Option<i64>,
    pub name: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl Database{
    /// Define Database defaults
    ///
    /// # Returns
    /// `Database`
    pub fn default() -> Database{
        Database{
            host: None,
            port: None,
            name: None,
            username: None,
            password: None
        }
    }
}

#[derive(Clone, Debug)]
pub struct Auth {
    pub auth_secret: Option<String>,
    pub claim_iss: Option<String>
}

impl Auth{
    /// Define Database defaults
    ///
    /// # Returns
    /// `Database`
    pub fn default() -> Auth{
        Auth{
            auth_secret: None,
            claim_iss: None
        }
    }
}

impl Config{

    /// Create A Config struct
    ///
    /// # Returns
    /// `Config` - Config struct
    pub fn new() -> Config{

        let mut database = None;
        let mut auth = None;

        match read_config_from_file("config/config.toml"){
            Ok(config_string) => {
                //debug!("Config String: {}", configString);
                match toml::Parser::new(&config_string).parse(){
                    Some(config_table) => {
                        database = parse_database_config(config_table.clone());
                        auth = parse_auth_config(config_table.clone());
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
                Some(d) => d,
                None => Database::default()
            },
            auth:match auth{
                Some(a) => a,
                None => Auth::default()
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

/// Parse authentication configuration
///
/// # Arguments
/// * `config_table` - toml::Table The toml file represented as a BTreeMap
///
/// # Returns
/// `Option<Auth>` - Auth Config
fn parse_auth_config(config_table: toml::Table) -> Option<Auth>{
    match config_table.get("auth"){
        Some(ac) => {
            match ac.as_table(){
                Some(auth_config) => {
                    //std::collections::BTreeMap
                    let auth_secret = match auth_config.get("auth_secret"){
                        Some(v) => {
                            //v: toml::Value
                            match v.as_str(){
                                Some(vs) => Some(vs.to_string()),
                                None => {
                                    warn!("Cannot read authentication auth_secret as string");
                                    None
                                }
                            }
                        },
                        None => {
                            warn!("Authentication auth_secret not found in configuration");
                            None
                        }
                    };
                    let claim_iss = match auth_config.get("claim_iss"){
                        Some(v) => {
                            //v: toml::Value
                            match v.as_str(){
                                Some(vs) => Some(vs.to_string()),
                                None => {
                                    warn!("Cannot read authentication claim_iss as string");
                                    None
                                }
                            }
                        },
                        None => {
                            warn!("Authentication claim_iss not found in configuration");
                            None
                        }
                    };

                    Some(Auth{
                        auth_secret: auth_secret,
                        claim_iss: claim_iss
                    })
                },
                None => None
            }
        },
        None => {
            warn!("No `authentication` configuration found.");
            None
        }
    }
}
