use std::path::PathBuf;
use clap::{Parser, Subcommand as ClapSubcomand, Args};

/// A small PNG modifying utility
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub subcommand: Subcommand,
}

/// Available subcommands
#[derive(ClapSubcomand)]
pub(crate) enum Subcommand {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs)
}

/// Encoding PNG files with secret message
#[derive(Args)]
pub(crate) struct EncodeArgs {
    /// A path to PNG file
    pub file_path: PathBuf,
    pub chunk_type: String,
    pub message: String,
    pub output_file_path: PathBuf
}

/// Decoding PNG secrets to extract secrets
#[derive(Args)]
pub(crate) struct DecodeArgs {
    pub file_path: PathBuf,
    pub chunk_type: String
}

/// Removes selected chunk from PNG file
#[derive(Args)]
pub(crate) struct RemoveArgs {
    pub file_path: PathBuf,
    pub chunk_type: String
}

/// Print PNG file's chunks (their types)
#[derive(Args)]
pub(crate) struct PrintArgs {
    pub file_path: PathBuf
}