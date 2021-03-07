use std::process::Command;

use crate::structs::FFprobeMetadata;

pub struct FFmpeg {}

impl FFmpeg {
    pub fn get_metadata(file_path: &str) -> Result<FFprobeMetadata, String> {
        let output = Command::new("ffprobe")
            .args(&["-v", "quiet", "-print_format", "json", "-show_format", file_path])
            .output();

        match output {
            Ok(output) => match std::str::from_utf8(output.stdout.as_ref()) {
                Ok(json) => match serde_json::from_str::<FFprobeMetadata>(json) {
                    Ok(metadata) => Ok(metadata),
                    Err(e) => Err(e.to_string())
                },
                Err(e) => Err(e.to_string())
            },
            Err(e) => Err(e.to_string())
        }
    }
}
