use audiotags::Tag;
use std::path::PathBuf;

pub fn set_metadata(
    artist: String,
    path: PathBuf,
    album_name: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let guess = mime_guess::from_path(path.to_str().unwrap());
    let mimetype = guess.first().unwrap().to_string();

    if !mimetype.starts_with("audio") {
        return Err(
            "The specified file is not an audio file. Please provide a valid audio file.".into(),
        );
    }

    let mut tag = Tag::new().read_from_path(&path)?;

    let audio_name = &path.to_str().unwrap().split('/').last().unwrap();
    let song_title = audio_name.trim_end_matches(".mp3").trim_end_matches(".mp4");

    tag.set_artist(&artist);
    println!(
        "Info: Title set to '{}' (derived from audio file's name)",
        &song_title
    );
    tag.set_title(&song_title);
    tag.set_album_title(&album_name);
    tag.write_to_path(&path.to_str().unwrap())?;

    println!(
        "Info: Metadata set successfully for file: {}",
        path.display()
    );

    Ok(())
}
