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

    let specified_path = Path::new(args.path.as_os_str());

    println!("Info: Artist found - '{}'", args.artist);

    if *is_album {
        if specified_path.is_file() {
            return Err("path is a file! (try removing the '--album' option)".into());
        }

        let mut album_name = &args.path.to_str().unwrap();

        let album_name = album_name.trim_end_matches('/').split('/').last().unwrap();

        println!(
            "Info: Album name detected as '{}' (based on the parent folder name).",
            album_name
        );

        for song in fs::read_dir(&args.path.as_os_str())? {
            set_metadata(
                args.artist.to_string(),
                song.unwrap().path().to_path_buf(),
                album_name.to_string(),
            );
        }
    } else {
        if specified_path.is_dir() {
            return Err("The specified path is a directory. Please use the '--album' option for album directories.".into());
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
            "Info: Album name detected as '{}' (based on the parent folder name).",
            album_name
        );

        set_metadata(args.artist, args.path.to_path_buf(), album_name.to_string());
    }

    Ok(())
}
