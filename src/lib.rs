#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]
extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate hyper;


#[macro_use]
extern crate log;

pub mod client;
pub mod data;
mod error;
mod http;
