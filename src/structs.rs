use clap::crate_version;
use crate::{ffmpeg::FFmpeg, enums::OpusApplication};
use serde::{Serialize, Deserialize};

pub const DCA_VERSION: u8 = 1;

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    dca: DCAMetadata,
    info: SongMetadata,
    origin: OriginMetadata,
    opus: OpusMetadata,
}

impl Metadata {
    pub(crate) fn new(file_path: &str, bitrate: u32, application: OpusApplication, sample_rate: u32, frame_size: u32, channels: u8) -> Result<Self, String> {
        let mut metadata = Self {
            dca: DCAMetadata {
                version: DCA_VERSION,
                tool: DCAToolMetadata {
                    name: String::from("dca-rs"),
                    version: String::from(crate_version!()),
                    url: String::from("https://github.com/davidcole1340/dca-rs"),
                    author: String::from("David Cole <david.cole1340@gmail.com>"),
                },
            },
            info: SongMetadata {
                title: None,
                artist: None,
                album: None,
                genre: None,
                comments: None,
                cover: None,
            },
            origin: OriginMetadata {
                source: Some(file_path.to_string()),
                bitrate: None,
                channels,
                encoding: None,
                url: None,
            },
            opus: OpusMetadata {
                bitrate,
                sample_rate,
                application: application.to_string(),
                frame_size,
                channels,
            }
        };

        if let Ok(meta) = FFmpeg::get_metadata(file_path) {
            metadata.info.title = meta.format.tags.title;
            metadata.info.artist = meta.format.tags.artist;
            metadata.info.album = meta.format.tags.album;
            metadata.info.genre = meta.format.tags.genre;
            metadata.origin.encoding = Some(meta.format.format_long_name);
            metadata.origin.bitrate = Some(meta.format.bitrate.parse().unwrap());
        }

        Ok(metadata)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DCAMetadata {
    version: u8,
    tool: DCAToolMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DCAToolMetadata {
    name: String,
    version: String,
    url: String,
    author: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SongMetadata {
    title: Option<String>,
    artist: Option<String>,
    album: Option<String>,
    genre: Option<String>,
    comments: Option<String>,
    cover: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OriginMetadata {
    source: Option<String>,
    #[serde(rename = "abr")]
    bitrate: Option<u32>,
    channels: u8,
    encoding: Option<String>,
    url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpusMetadata {
    bitrate: u32,
    sample_rate: u32,
    application: String,
    frame_size: u32,
    channels: u8,
}

// FFprobe pub structs

#[derive(Debug, Serialize, Deserialize)]
pub struct FFprobeMetadata {
    format: FFprobeFormat,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FFprobeFormat {
    filename: String,
    #[serde(rename = "nb_streams")]
    num_streams: i32,
    #[serde(rename = "nb_programs")]
    num_programs: i32,
    format_name: String,
    format_long_name: String,
    start_time: String,
    duration: String,
    size: String,
    #[serde(rename = "bit_rate")]
    bitrate: String,
    probe_score: u32,
    tags: FFprobeTags,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FFprobeTags {
    date: Option<String>,
    track: Option<String>,
    artist: Option<String>,
    genre: Option<String>,
    title: Option<String>,
    album: Option<String>,
    compilation: Option<String>,
}
