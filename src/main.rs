use chunk_type::ChunkType;
use clap::{AppSettings, Clap};
use std::{convert::TryFrom, fs, path::PathBuf};

mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clap)]
struct Opts {
    #[clap(subcommand)]
    subcmd: Subcommand,
}

#[derive(Clap)]
enum Subcommand {
    Encode(Encode),
    Decode(Decode),
    Remove(Remove),
    Print(Print),
}

#[derive(Clap)]
struct Encode {
    file_path: PathBuf,
    chunk_type: ChunkType,
    message: String,
    output_file: Option<PathBuf>,
}

#[derive(Clap)]
struct Decode {
    file_path: PathBuf,
    chunk_type: ChunkType,
}

#[derive(Clap)]
struct Remove {
    file_path: PathBuf,
    chunk_type: ChunkType,
}

#[derive(Clap)]
struct Print {
    file_path: PathBuf,
}

fn main() -> Result<()> {
    let opts = Opts::parse();
    match opts.subcmd {
        Subcommand::Encode(v) => {
            let bytes = fs::read(&v.file_path)?;
            let mut png = png::Png::try_from(&bytes[..])?;
            let data: Vec<u8> = v.message.as_bytes().to_vec();
            let chunk = chunk::Chunk::new(v.chunk_type, data);
            png.append_chunk(chunk);
            match v.output_file {
                Some(path) => fs::write(path, png.as_bytes())?,
                None => fs::write(&v.file_path, png.as_bytes())?,
            }
        }
        Subcommand::Decode(v) => {}
        Subcommand::Remove(v) => {}
        Subcommand::Print(v) => {}
        _ => {
            println!("Invalid subcommand")
        }
    }

    Ok(())
}
