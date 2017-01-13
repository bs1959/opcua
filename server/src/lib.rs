#[macro_use]
extern crate log;

extern crate chrono;

extern crate serde;
extern crate serde_yaml;

extern crate opcua_core;
extern crate byteorder;

pub mod config {
    include!(concat!(env!("OUT_DIR"), "/config.rs"));
}

mod handshake;
mod services;

mod server;
pub use server::*;

pub mod tcp_session;
pub mod subscription;
pub mod monitored_item;

#[cfg(test)]
mod tests;