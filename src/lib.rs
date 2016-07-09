pub mod common;
pub mod dao;
pub mod models;
pub mod controllers;

#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;

extern crate rustc_serialize;

#[macro_use]
extern crate log;
extern crate log4rs;
extern crate nickel;
