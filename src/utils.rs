use crate::errors::Error;
use std::process::{Command, Stdio};

/// 执行
pub fn shell_exec(cmd: &str) -> Result<String, Error> {
    let shell = if cfg!(windows) {
        vec!["cmd".to_owned(), "/C".to_owned()]
    } else {
        vec!["sh".to_owned(), "-c".to_owned()]
    };

    let mut process = Command::new(&shell[0]);

    process.args(&shell[1..]).arg(cmd).stdin(Stdio::piped());
    // .stdout(Stdio::piped());

    let out = match process.output() {
        Ok(out) => out,
        Err(e) => {
            log::error!("Failed to start shell: {}", e);
            return Err(e.into());
        }
    };

    let output_str = String::from_utf8(out.stdout).unwrap();

    Ok(output_str)
}
