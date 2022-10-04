use std::fmt::{Display, Formatter};
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use crate::{Result, Error};
use crate::args::{Cli, Commands, DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};

use clap::Parser;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;

pub struct Handler{}

impl Handler {
    pub fn handle() -> Result<()> {
        let args = Cli::parse();
        match &args.command {
            Commands::Encode(arg) => Self::handle_encode(arg),
            Commands::Decode(arg) => Self::handle_decode(arg),
            Commands::Remove(arg) => Self::handle_remove(arg),
            Commands::Print(arg) => Self::handle_print(arg)
        }
    }

    fn handle_encode(args: &EncodeArgs) -> Result<()> {
        let mut png = Png::from_file(&args.file_path)?;
        let chunk_type = ChunkType::from_str(&args.chunk_type)?;
        let data = args.message.bytes().collect();

        let chunk = Chunk::new(chunk_type, data);

        png.append_chunk(chunk);

        let output = match &args.output_file {
            Some(output) => output,
            None => &args.file_path
        };

        fs::write(output, png.as_bytes())?;
        println!("Encoding successful!");
        Ok(())
    }

    fn handle_decode(args: &DecodeArgs) -> Result<()> {
        let png = Png::from_file(&args.file_path)?;
        let maybe_chunk = Png::chunk_by_type(&png, &args.chunk_type);
        match maybe_chunk {
            Some(chunk) => {
                let chunk_data = chunk.data_as_string()?;
                println!("{chunk_data}");
                Ok(())
            }
            None => {
                Err(Box::new(HandlerError::ChunkNotFound))
            }
        }
    }

    fn handle_remove(args: &RemoveArgs) -> Result<()> {
        let mut png = Png::from_file(&args.file_path)?;
        png.remove_chunk(&args.chunk_type)?;

        fs::write(&args.file_path, png.as_bytes())?;

        println!("Chunk removed!");
        Ok(())
    }

    fn handle_print(args: &PrintArgs) -> Result<()> {
        let png = Png::from_file(&args.file_path)?;
        println!("{}", png);
        Ok(())
    }
}

#[derive(Debug)]
enum HandlerError {
    ChunkNotFound
}

impl Display for HandlerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HandlerError::ChunkNotFound => {
                write!(f, "Chunk was not found!")
            }
        }
    }
}

impl std::error::Error for HandlerError {}