extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate sodiumoxide;
extern crate rocksdb;
extern crate structopt;
extern crate clap;
extern crate mitrid_core;

pub mod crypto;
pub mod io;
pub mod model;
pub mod app;

pub const VERSION: &str = "0.1.0";
