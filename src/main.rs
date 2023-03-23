#![allow(unused)]

// ps: find a way to remove all of those unwraps
use anyhow::{anyhow, Context, Result};
use audiotags::Tag;
use clap::{arg, Command, Parser};
use std::{
    fmt::Error,
    fs::{self, read_to_string},
    ops::Index,
    path::{self, Path, PathBuf},
};

#[derive(Parser)]
#[command(author, version)]
struct Argument {
    /// The artist's name
    #[arg(short, long)]
    artist: String,

    /// Whether or not the program is dealing with an entire album (automatically sets the metadata album as the specified --path)
    #[arg(short = 'A', long, default_value_t = false)]
    album: bool,

    /// Path to the music file, if '--album' is used this path should point to a folder instead
    path: PathBuf,
}

#[derive(Debug)]
struct PathIsDirectoryError(String);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Argument::parse();

    let is_album = &args.album;

    println!("info: Artist is '{}' \n", args.artist);

    if *is_album == true {
        let path = Path::new(args.path.as_os_str());
        if path.is_file() {
            return Err("path is a file! (try removing the '--album' option)".into());
        }

        let mut album_name = &args.path.to_str().unwrap();

        let album_name = album_name.trim_end_matches('/').split('/').last().unwrap();

        println!("info: Album is '{}' (parent folder's name) \n", album_name);

        for song in fs::read_dir(&args.path.as_os_str()).unwrap() {
            set_metadata(
                args.artist.to_string(),
                song.unwrap().path().to_path_buf(),
                album_name.to_string(),
            );
        }
    } else {
        if Path::new(&args.path.as_os_str()).is_dir() {
            return Err("path is a directory! (try using the '--album' option)".into());
        }

        let album_name = &args
            .path
            .parent()
            .unwrap()
            .to_str()
            .unwrap()
            .split('/')
            .last()
            .unwrap();

        println!(
            "info: Album is: '{}' (parent folder's name)",
            album_name.to_string()
        );

        set_metadata(args.artist, args.path.to_path_buf(), album_name.to_string());
    }

    Ok(())
}

fn set_metadata(artist: String, path: PathBuf, album_name: String) {
    let guess = mime_guess::from_path(path.to_str().unwrap());
    let mimetype = guess.first().unwrap().to_string();

    if !mimetype.starts_with("audio") {
        return;
    }

    let mut tag = Tag::new().read_from_path(&path).unwrap();

    let audio_name = &path.to_str().unwrap().split('/').last().unwrap();

    let audio_name_splited: Vec<&str> = audio_name.split('.').collect();
    let song_title = audio_name_splited.first().unwrap();

    tag.set_artist(&artist);
    println!("info: Title is '{}' (audio file's name)", &song_title);
    tag.set_title(&song_title);
    tag.set_album_title(&album_name);
    tag.write_to_path(&path.to_str().unwrap()).unwrap();

    println!(
        "info: Succesfully set metadata for '{}' \n",
        &path.to_str().unwrap()
    );
}
