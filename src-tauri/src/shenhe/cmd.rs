use std::io::{self, Write};
use std::process::Command;

fn run_command(name: &str, args: &[&str]) -> Result<String, String> {
    let output = Command::new(name)
        .args(args)
        .output()
        .map_err(|err| format!("{}: {}", name, err))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if !stdout.is_empty() {
        io::stdout().write_all(stdout.as_bytes()).unwrap();
    }

    if !stderr.is_empty() {
        io::stderr().write_all(stderr.as_bytes()).unwrap();
    }

    if output.status.success() {
        Ok(stdout)
    } else {
        Err(stderr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_run_command() {
        match run_command("ebook-convert", &["--version"]) {
            Ok(output) => assert!(output.contains("calibre")),
            Err(error) => assert_eq!(
                error, "",
                "did you installed calibre? assertion failed with error: {}",
                error
            ),
        }
    }
}
