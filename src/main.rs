use chunk_type::ChunkType;
use clap::Clap;
use std::{convert::TryFrom, fs, path::PathBuf, str};

mod chunk;
mod chunk_type;
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

            println!("Successfully encoded.");
        }
        Subcommand::Decode(v) => {
            let bytes = fs::read(&v.file_path)?;
            let png = png::Png::try_from(&bytes[..])?;
            if let Some(c) = png.chunk_by_type(str::from_utf8(&v.chunk_type.data).unwrap()) {
                println!("Decoded: {}", c.data_as_string()?);
            } else {
                println!("Nothing found.");
            }
        }
        Subcommand::Remove(v) => {
            let bytes = fs::read(&v.file_path)?;
            let mut png = png::Png::try_from(&bytes[..])?;
            png.remove_chunk(str::from_utf8(&v.chunk_type.data).unwrap())?;

            println!("Successfully removed.");
        }
        Subcommand::Print(v) => {
            let bytes = fs::read(&v.file_path)?;
            let png = png::Png::try_from(&bytes[..])?;
            println!("{}", png);
        }
        _ => {
            println!("Invalid subcommand!")
        }
    }

    Ok(())
}
