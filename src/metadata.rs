use audiotags::{AudioTag, Tag};
use std::{path::PathBuf, string};

use crate::errs::FILE_IS_NOT_AUDIO;

pub fn set_metadata(
    artist: String,
    path: PathBuf,
    album_name: String,
    specified_song_title: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let guess = mime_guess::from_path(path.to_str().unwrap());
    let mimetype = guess.first().unwrap().to_string();

    if !mimetype.starts_with("audio") {
        return Err(FILE_IS_NOT_AUDIO.into());
    }

    let mut tag = Tag::new().read_from_path(&path)?;

    let mut song_title = "";

    if specified_song_title.eq("undefined") {
        let audio_name = &path.to_str().unwrap().split('/').last().unwrap();
        let song_title = audio_name.trim_end_matches(".mp3").trim_end_matches(".mp4");

        println!(
            "Info: Title set to '{}' (derived from audio file's name)",
            &song_title
        );
    } else {
        let song_title = specified_song_title;

        println!(
            "Info: Title set to '{}' (manually set by user)",
            &song_title
        );
    }

    tag.set_title(&song_title);
    tag.set_artist(&artist);
    tag.set_album_title(&album_name);
    tag.write_to_path(&path.to_str().unwrap())?;
    Ok(())
}

pub fn get_metadata(path: PathBuf) -> Result<Box<dyn AudioTag>, String> {
    let guess = mime_guess::from_path(path.to_str().unwrap());
    let mimetype = guess.first().unwrap().to_string();

    if !mimetype.starts_with("audio") {
        return Err(FILE_IS_NOT_AUDIO.into());
    }

    let mut tag = Tag::new().read_from_path(&path).unwrap();

    Ok(tag)
}
