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
pub mod handlers;
pub mod logger;
pub mod models;
pub mod state;
