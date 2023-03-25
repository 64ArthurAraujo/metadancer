#![allow(unused)]

mod argument;
mod metadata;

use anyhow::{anyhow, Context, Result};
use audiotags::Tag;
use clap::Parser;
use std::{
    fmt::Error,
    fs::{self, read_to_string},
    ops::Index,
    path::{self, Path, PathBuf},
};

use crate::{argument::Argument, metadata::set_metadata};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Argument::parse();

    let is_album = &args.album;

    let especified_path = Path::new(args.path.as_os_str());

    println!("Info: Artist is '{}'", args.artist);

    if *is_album == true {
        if especified_path.is_file() {
            return Err("path is a file! (try removing the '--album' option)".into());
        }

        let mut album_name = &args.path.to_str().unwrap();

        let album_name = album_name.trim_end_matches('/').split('/').last().unwrap();

        println!("Info: Album is '{}' (parent folder's name) \n", album_name);

        for song in fs::read_dir(&args.path.as_os_str())? {
            set_metadata(
                args.artist.to_string(),
                song.unwrap().path().to_path_buf(),
                album_name.to_string(),
            );
        }
    } else {
        if especified_path.is_dir() {
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
            "Info: Album is: '{}' (parent folder's name)",
            album_name.to_string()
        );

        set_metadata(args.artist, args.path.to_path_buf(), album_name.to_string());
    }

    Ok(())
}
