# Metadancer
A basic cli tool for editing metadata of a lot of audio files at once.

## Usage
The program assumes that the folder you specify is the name of the album and the audio name is the song's name, to have it work perfectly the assumed structure is:
```sh
.
├── (ALBUM)
│   └── (SONG).mp3
```
#### Example:
If we have an entire album folder:
```sh
└── mirror【限定盤】
    ├── Cover.jpg
    ├── Englishman in New York.mp3
    ├── mirror.mp3
    ├── morrow.mp3
    ├── ダージリン.mp3
    └── 彗星のパレード.mp3
```
The command should be:
```sh
# --a (artist's name)
# --album (Indicates that we are dealing with an entire album folder)
metadancer --a まじ娘 --album ./mirror【限定盤】
```

For more use cases type `metadancer --help`
