use std::path::PathBuf;

use clap::{arg, Command, Parser};

#[derive(Parser)]
#[command(author, version)]
pub struct Argument {
    /// The artist's name
    #[arg(short, long)]
    pub artist: String,

    /// Whether or not the program is dealing with an entire album (automatically sets the metadata album as the specified --path)
    #[arg(short = 'A', long, default_value_t = false)]
    pub album: bool,

    /// Path to the music file, if '--album' is used this path should point to a folder instead
    pub path: PathBuf,
}
