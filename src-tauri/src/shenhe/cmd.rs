// use std::io::{self, Write};
use super::types::ProgressReporter;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use std::process::Command;
use tauri::{Runtime, Wry};
#[cfg(target_os = "windows")]
use winapi::um::winbase::CREATE_NO_WINDOW;

pub fn run_command<R: Runtime>(
    name: &str,
    _reporter: Option<&ProgressReporter<R>>,
    args: &[&str],
) -> Result<String, String> {
    let mut command = Command::new(name);
    command.args(args);
    #[cfg(target_os = "windows")]
    {
        command.creation_flags(CREATE_NO_WINDOW);
    }
    let output = command.output().map_err(|err| {
        if name == "ebook-convert" {
            return String::from("please install calibre first, click the 💗 on the left to open the About dialog, you can find the download URL there.");
        }
        return format!("{}: {}", name, err);
    })?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    // if !stdout.is_empty() {
    //     io::stdout().write_all(stdout.as_bytes()).unwrap();
    // }

    // if !stderr.is_empty() {
    //     io::stderr().write_all(stderr.as_bytes()).unwrap();
    // }

    if output.status.success() {
        Ok(stdout)
    } else {
        Err(stderr)
    }
}

pub fn ebook_convert_exists() -> bool {
    let reporter: Option<&ProgressReporter<Wry>> = None;
    run_command("ebook-convert", reporter, &["--version"]).is_ok()
}

#[cfg(test)]
mod tests {
    use tauri::Wry;

    use super::*;
    #[test]
    fn test_run_command() {
        let reporter: Option<&ProgressReporter<Wry>> = None;
        match run_command("ebook-convert", reporter, &["--version"]) {
            Ok(output) => assert!(output.contains("calibre")),
            Err(error) => assert_eq!(
                error, "",
                "did you installed calibre? assertion failed with error: {}",
                error
            ),
        }
    }

    #[test]
    fn test_ebook_convert_exists() {
        assert!(ebook_convert_exists());
    }
}
