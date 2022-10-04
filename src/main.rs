extern crate core;

mod chunk_type;
mod chunk;
mod png;
mod args;
mod commands;

pub const MAX_CHUNK_LEN: u32 = 2147483648;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    if let Err(e) = commands::Handler::handle() {
        eprintln!("Handler error: {e}");
    }
}
