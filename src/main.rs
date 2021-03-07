pub mod structs;
pub mod enums;
pub mod validators;
pub mod ffmpeg;

use std::process::exit;

use clap::{Clap, ValueHint};
use enums::OpusApplication;
use structs::Metadata;
use validators::*;

#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "David Cole <david.cole1340@gmail.com>")]
struct Options {
    #[clap(value_hint = ValueHint::FilePath, default_value = "pipe:0", validator = in_file_validator)]
    in_file: String,

    #[clap(long, default_value = "encode", possible_values = &["encode"])]
    mode: String,
    #[clap(long, default_value = "256", validator = volume_validator)]
    volume: u16,
    #[clap(long, default_value = "2", possible_values = &["1", "2"])]
    channels: u8,
    #[clap(long, default_value = "48000")]
    frame_rate: u32,
    #[clap(long, default_value = "960", possible_values = &["960", "1920", "2880"])]
    frame_size: u32,
    #[clap(long, default_value = "64", validator = bitrate_validator)]
    bitrate: u32,
    #[clap(long, default_value = "audio", possible_values = &["audio", "voip", "lowdelay"])]
    application: OpusApplication,
    #[clap(long, default_value = "jpeg", possible_values = &["jpeg"])]
    cover_format: String,
    #[clap(long)]
    raw: bool,
}

fn main() {
    let opts: Options = Options::parse();

    // check for FFmpeg and FFprobe
    if let Err(e) = check_for_executable("ffmpeg", "FFmpeg") {
        eprintln!("{}", e);
        exit(1);
    }

    if let Err(e) = check_for_executable("ffprobe", "FFprobe") {
        eprintln!("{}", e);
        exit(1);
    }

    let metadata = Metadata::new(opts.in_file.as_str(), opts.bitrate * 1000, opts.application, opts.frame_rate, opts.frame_size, opts.channels);
}
