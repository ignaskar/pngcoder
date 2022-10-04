use std::path::PathBuf;
use clap::{Args, Parser, Subcommand};
use std::ffi::OsString;

#[derive(Debug, Parser)]
#[command(name = "pngc")]
#[command(about = "Encode/decode messages in PNG files", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Args, Debug)]
pub struct EncodeArgs {
    pub file_path: PathBuf,
    pub chunk_type: String,
    pub message: String,
    #[arg(value_parser = clap::value_parser!(OsString), short, long)]
    pub output_file: Option<PathBuf>
}

#[derive(Args, Debug)]
pub struct DecodeArgs {
    pub file_path: PathBuf,
    pub chunk_type: String
}

#[derive(Args, Debug)]
pub struct RemoveArgs {
    pub file_path: PathBuf,
    pub chunk_type: String
}

#[derive(Args, Debug)]
pub struct PrintArgs {
    pub file_path: PathBuf
}
