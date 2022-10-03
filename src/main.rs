extern crate core;

mod chunk_type;
mod chunk;
mod png;

pub const MAX_CHUNK_LEN: u32 = 2147483648;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    todo!()
}
