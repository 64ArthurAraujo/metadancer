#![allow(unused)]

mod argument;
mod metadata;
mod errs;

use anyhow::{anyhow, Context, Result};
use audiotags::{Tag, Album};
use clap::Parser;
use errs::COULDNT_GET_USER;
use users::get_current_username;
use walkdir::WalkDir;
use std::ffi::OsString;
use std::os::unix::ffi::OsStringExt;
use std::{
    fmt::Error,
    fs::{self, read_to_string},
    ops::Index,
    path::{self, Path, PathBuf},
};

use crate::{argument::Argument, metadata::{set_metadata, get_metadata}, errs::{PATH_IS_DIR, PATH_IS_FILE, ARTIST_IS_REQUIRED}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Argument::parse();

    let is_album = &args.album;

    let is_reading_artist = &args.read_artist;
    let is_reading_album = &args.read_album;
    let is_reading_title = &args.read_title;

    let is_doing_lookup = &args.lookup;

    let specified_path = Path::new(args.path.as_os_str());

    if *is_doing_lookup {
        for entry in WalkDir::new(specified_path).into_iter().filter_map(Result::ok) {
            if entry.file_type().is_file() {
                if let Some(extension) = entry.path().extension() {
                    if extension == "mp3" {
                        println!("{}", entry.path().display());
                    }
                }
            }
        }
    } else if *is_reading_artist {
        if specified_path.is_dir() {
            return Err(PATH_IS_DIR.into());
        }

        let audio = get_metadata(specified_path.to_path_buf()).unwrap();

        println!("Artist: {}", audio.artist().unwrap());
    } else if *is_reading_album {
        if specified_path.is_dir() {
            return Err(PATH_IS_DIR.into());
        }

        let audio = get_metadata(specified_path.to_path_buf()).unwrap();

        println!("Album: {}", audio.album_title().unwrap());
    } else if *is_reading_title {
        if specified_path.is_dir() {
            return Err(PATH_IS_DIR.into());
        }

        let audio = get_metadata(specified_path.to_path_buf()).unwrap();

        println!("Title: {}", audio.title().unwrap());
    } else if *is_album {
        if args.artist.eq("undefined") {
            return Err(ARTIST_IS_REQUIRED.into());
        }

        println!("Info: Artist found - '{}'", args.artist);

        if specified_path.is_file() {
            return Err(PATH_IS_FILE.into());
        }

        let mut album_name = "";

        if args.album_title.eq("undefined") {
            let album_name = &args.path.to_str().unwrap();

            let album_name = album_name.trim_end_matches('/').split('/').last().unwrap();

            println!(
                "Info: Album name detected as '{}' (based on the parent folder name).",
                album_name
            );
        } else {
            let album_name = &args.album_title;

            println!(
                "Info: Album name detected as '{}' (manually set by user).",
                album_name
            );
        }

        for song in fs::read_dir(&args.path.as_os_str())? {
            set_metadata(
                args.artist.to_string(),
                song.unwrap().path().to_path_buf(),
                album_name.to_string(),
            );
        }
    } else {
        if args.artist.eq("undefined") {
            return Err(ARTIST_IS_REQUIRED.into());
        }

        println!("Info: Artist found - '{}'", args.artist);

        if specified_path.is_dir() {
            return Err(PATH_IS_DIR.into());
        }

        let mut album_name = "";

        if args.album_title.eq("undefined") {
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
        } else {
            let album_name = &args.album_title;

            println!(
                "Info: Album name detected as '{}' (manually set by user).",
                album_name
            );
        }

        set_metadata(args.artist, args.path.to_path_buf(), album_name.to_string());
    }

    Ok(())
}
