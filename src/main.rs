use chunk::Chunk;
use clap::Parser;
use png::Png;
use std::fs;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> anyhow::Result<()> {
    let args = args::PngArgs::parse();

    match args.command {
        args::Command::Encode {
            file_path,
            chunk_type,
            message,
            output_file,
        } => {
            let bytes = fs::read(file_path)?;
            let mut png = Png::try_from(bytes.as_ref())?;

            let chunk = Chunk::new(chunk_type, message.try_into()?);
            png.append_chunk(chunk);

            println!("{}", png);

            if let Some(file) = output_file {
                fs::write(file, png.as_bytes())?;
            }
        },
        args::Command::Decode {
            file_path,
            chunk_type,
        } => todo!(),
        args::Command::Remove {
            file_path,
            chunk_type,
        } => todo!(),
        args::Command::Print { path } => {
            let bytes = fs::read(path)?;
            let png = Png::try_from(bytes.as_ref())?;

            println!("{}", png);
        },
    }

    Ok(())
}
