use std::{io::Write, process::Command};

use anyhow::Result;

pub fn clang_format(input: String) -> Result<String> {
    let mut cmd = Command::new("clang-format")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()?;
    cmd.stdin.as_mut().unwrap().write_all(input.as_bytes())?;
    let output = cmd.wait_with_output()?;
    Ok(String::from_utf8(output.stdout)?)
}
