use std::{path::Path, process::Command};

pub fn in_file_validator(file: &str) -> Result<(), String> {
    if file == "pipe:0" || Path::new(file).exists() {
        Ok(())
    } else {
        Err(format!("Unable to find file {}, use 'pipe:0' for stdin", file))
    }
}

pub fn volume_validator(vol: &str) -> Result<(), String> {
    match vol.parse::<i32>() {
        Ok(vol) if vol >= 0 && vol <= 256 => Ok(()),
        _ => Err(format!("Volume must be a valid integer between 0 and 256, {} given", vol))
    }
}

pub fn check_for_executable(exec_name: &str, friendly_name: &str) -> Result<(), String> {
    let cmd = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "where", exec_name])
            .output()
    } else {
        Command::new("sh")
            .args(&["-c", "command", "-v", exec_name])
            .output()
    };

    match cmd {
        Ok(cmd) => match cmd.status.code() {
            Some(code) if code < 1 => Ok(()),
            _ => Err(format!("Could not locate {}", friendly_name))
        },
        Err(e) => Err(format!("Could not spawn process: {}", e))
    }
}
