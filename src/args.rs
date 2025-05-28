use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::chunk_type::ChunkType;

#[derive(Debug, Parser)]
#[clap(name = "pngme", version)]
pub struct PngArgs {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Encode {
        file_path: PathBuf,
        chunk_type: ChunkType,
        message: String,
        output_file: Option<PathBuf>,
    },
    Decode {
        file_path: PathBuf,
        chunk_type: ChunkType,
    },
    Remove {
        file_path: PathBuf,
        chunk_type: ChunkType,
    },
    Print {
        path: PathBuf,
    },
}
