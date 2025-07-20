#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate tracing;
#[macro_use]
extern crate validator;

pub mod config;
pub mod entity;
pub mod errors;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod state;
pub mod utils;
