mod argument;
mod errs;
mod metadata;

use anyhow::Result;
use clap::Parser;
use std::{fs, path::Path};
use walkdir::WalkDir;

use crate::{
    argument::Argument,
    errs::{ARTIST_IS_REQUIRED, PATH_IS_DIR, PATH_IS_FILE},
    metadata::{get_metadata, set_metadata},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Argument::parse();

    let is_album = &args.album;
    let is_reading_artist = &args.read_artist;
    let is_reading_album = &args.read_album;
    let is_reading_title = &args.read_title;
    let is_doing_lookup = &args.lookup;

    let specified_path = Path::new(args.path.as_os_str());
    let specified_song_title = &args.song_title;

    if *is_doing_lookup {
        for entry in WalkDir::new(specified_path)
            .into_iter()
            .filter_map(Result::ok)
        {
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

        let album_name = "";

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
            let _ = set_metadata(
                args.artist.to_string(),
                song.unwrap().path().to_path_buf(),
                album_name.to_string(),
                specified_song_title.to_string(),
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

        let album_name = "";

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

        let _ = set_metadata(
            args.artist,
            args.path.to_path_buf(),
            album_name.to_string(),
            specified_song_title.to_string(),
        );
    }

    Ok(())
}
