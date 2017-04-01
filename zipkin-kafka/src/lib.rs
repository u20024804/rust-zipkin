#[macro_use]
extern crate error_chain;
extern crate kafka;

extern crate zipkin_core;

pub mod errors;
mod transport;

pub use transport::{KafkaConfig, KafkaTransport};