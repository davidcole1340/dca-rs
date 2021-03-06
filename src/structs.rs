use serde::{Serialize};

pub const VERSION: u32 = 1;

#[derive(Serialize)]
pub struct Metadata {
    dca: DCAMetadata,
    #[serde(rename = "info")]
    song_info: SongMetadata,
    origin: OriginMetadata,
    opus: OpusMetadata,
}

#[derive(Serialize)]
pub struct DCAMetadata {
    version: u8,
    tool: DCAToolMetadata,
}

#[derive(Serialize)]
pub struct DCAToolMetadata {
    name: String,
    version: String,
    url: String,
    author: String,
}

#[derive(Serialize)]
pub struct SongMetadata {
    title: String,
    artist: String,
    album: String,
    genre: String,
    comments: String,
    cover: String,
}

#[derive(Serialize)]
pub struct OriginMetadata {
    source: String,
    #[serde(rename = "abr")]
    bitrate: u32,
    channels: u32,
    encoding: String,
    url: String,
}

#[derive(Serialize)]
pub struct OpusMetadata {
    bitrate: u32,
    sample_rate: u32,
    application: String,
    frame_size: u32,
    channels: u32,
}

// FFprobe pub structs

#[derive(Serialize)]
pub struct FFprobeMetadata {
    format: FFprobeFormat,
}

#[derive(Serialize)]
pub struct FFprobeFormat {
    filename: String,
    num_streams: i32,
    num_programs: i32,
    format_name: String,
    format_long_name: String,
    start_time: String,
    duration: String,
    size: String,
    bitrate: String,
    probe_score: String,
    tags: FFprobeTags,
}

#[derive(Serialize)]
pub struct FFprobeTags {
    date: String,
    track: String,
    artist: String,
    genre: String,
    title: String,
    album: String,
    compilation: String,
}
