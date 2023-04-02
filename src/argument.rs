use std::path::PathBuf;

use clap::{arg, Command, Parser};

#[derive(Parser)]
#[command(author, version)]
pub struct Argument {
    /// The artist's name
    #[arg(short, long)]
    pub artist: String,

    /// Indicates whether the program is handling a single audio file or an entire album.
    /// If true, the program will automatically set the album metadata to the specified path.
    #[arg(short = 'A', long, default_value_t = false)]
    pub album: bool,

    /// Path to the music file.
    /// If '--album' is used, this path should point to a folder that contains the music files.
    pub path: PathBuf,
}
