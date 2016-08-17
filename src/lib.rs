pub mod common;
pub mod controllers;
pub mod dao;
pub mod models;

#[macro_use(bson, doc)]
extern crate bson;

#[macro_use]
extern crate log;
extern crate log4rs;

extern crate mongodb;

#[macro_use]
extern crate nickel;

extern crate rustc_serialize;
