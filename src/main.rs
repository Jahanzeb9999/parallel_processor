use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::task;

mod pipeline;
mod worker;
mod data;
mod error;


fn main() {
    println!("Hello, world!");
}
