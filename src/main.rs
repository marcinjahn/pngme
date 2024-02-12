use std::fs;
use clap::Parser;
use crate::args::{Cli, Subcommand};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;

mod args;
mod chunk;
mod chunk_type;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.subcommand {
        Subcommand::Encode(args) => {
            let file = fs::read(args.file_path)?;
            let chunk_type = args.chunk_type.as_bytes();

            if chunk_type.len() != 4 {
                panic!("Chunk type is wrong, it needs to be 4 ASCII bytes");
            }
            let chunk_type_bytes: [u8; 4] = args.chunk_type.as_bytes().try_into()?;
            let mut png = Png::try_from(&file[..])?;
            png.append_chunk(
                Chunk::new(
                    ChunkType::try_from(chunk_type_bytes)?,
                    args.message.as_bytes().to_vec()));

            fs::write(args.output_file_path, png.as_bytes())?;
        },
        Subcommand::Decode(args) => {
            let file = fs::read(args.file_path)?;
            let png = Png::try_from(&file[..])?;

            let chunk = png.chunk_by_type(&args.chunk_type);

            if let None = chunk {
                println!("Chunk not found");
                return Ok(());
            }

            println!("Decoded message: {}", chunk.unwrap().data_as_string()?);
        },
        Subcommand::Remove(args) => {
            let file = fs::read(&args.file_path)?;
            let mut png = Png::try_from(&file[..])?;
            png.remove_chunk(&args.chunk_type)?;

            fs::write(args.file_path, png.as_bytes())?;
        },
        Subcommand::Print(args) => {
            let file = fs::read(args.file_path)?;
            let png = Png::try_from(&file[..])?;

            println!("Chunk types in this file:");
            png.chunks().iter().for_each(|chunk| {
                println!("{}", chunk.chunk_type());
            });
        }
    }

    Ok(())
}
