pub mod address;
pub mod config;
pub mod heap;

pub use {address::*, config::*};

pub fn init() {
    heap::init();
    println!("mod memory initialized");
}
