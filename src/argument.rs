use std::path::PathBuf;

use clap::{arg, Parser};

#[derive(Parser)]
#[command(author, version)]
pub struct Argument {
    /// The artist's name
    #[arg(short, long, default_value = "undefined")]
    pub artist: String,

    /// Indicates whether the program is handling a single audio file or an entire album.
    /// If true, the program will automatically set the album metadata to the specified path.
    #[arg(short = 'A', long, default_value_t = false)]
    pub album: bool,

    #[arg(short = 't', long, default_value = "undefined")]
    pub album_title: String,

    /// Manually specify the song title (ignored if --album is used)
    #[arg(long, default_value = "undefined")]
    pub song_title: String,

    #[arg(long, default_value_t = false)]
    pub read_artist: bool,

    #[arg(long, default_value_t = false)]
    pub read_album: bool,

    #[arg(long, default_value_t = false)]
    pub read_title: bool,

    /// Looks through your system for mp3 files.
    #[arg(long, default_value_t = false)]
    pub lookup: bool,

    /// Path to the music file.
    /// If '--album' is used, this path should point to a folder that contains the music files.
    pub path: PathBuf,
}
