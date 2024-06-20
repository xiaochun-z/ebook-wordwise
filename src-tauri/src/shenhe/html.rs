use super::types::ProgressReporter;
use std::io::{self, Read, Seek, Write};
use tauri::Runtime;

pub fn process_html<R: Read + Seek, W: Write, Rt: Runtime>(
    reader: &mut R,
    writer: &mut W,
    process_text_fn: &(dyn Fn(&str) -> String),
    reporter: Option<&ProgressReporter<Rt>>,
) -> io::Result<()> {
    let mut buffer = [0; 1024];
    let mut inside_body = false;
    let mut inside_tag = false;
    let mut text_buffer = String::new();
    let mut tag_buffer = String::new();
    let mut incomplete_utf8 = Vec::new();

    // get total size of the reader to estimate the progress.
    let total_size: f64 = reader.seek(io::SeekFrom::End(0))? as f64;

    // reset to reader to begining to read
    reader.seek(io::SeekFrom::Start(0))?;

    let mut current_read: f64 = 0.0;

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        current_read += bytes_read as f64;
        if let Some(reporter) = reporter {
            let progress = current_read / total_size;
            //let progress = progress * 0.8 + 0.2;
            reporter.report(progress);
        }

        let mut slice = &buffer[..bytes_read];
        let cl = &incomplete_utf8.clone();
        // Handle incomplete UTF-8 character from the previous read
        if !incomplete_utf8.is_empty() {
            incomplete_utf8.extend_from_slice(slice);
            if let Ok(valid_str) = std::str::from_utf8(cl) {
                slice = valid_str.as_bytes();
                incomplete_utf8.clear();
            }
        }

        // Check if the slice ends with a partial UTF-8 character
        if !slice.is_empty() {
            let valid_up_to = match std::str::from_utf8(slice) {
                Ok(_) => slice.len(),
                Err(e) => e.valid_up_to(),
            };

            if valid_up_to < slice.len() {
                incomplete_utf8.extend_from_slice(&slice[valid_up_to..]);
                slice = &slice[..valid_up_to];
            }
        }

        let text = std::str::from_utf8(slice).expect("Slice should be valid UTF-8");

        for ch in text.chars() {
            if ch == '<' {
                inside_tag = true;
                if !text_buffer.is_empty() && inside_body {
                    let processed_text = process_text_fn(&text_buffer);
                    writer.write_all(processed_text.as_bytes())?;
                    text_buffer.clear();
                }
                tag_buffer.push(ch);
            } else if ch == '>' {
                inside_tag = false;
                tag_buffer.push(ch);

                // Check if we are entering or leaving the body tag
                if tag_buffer.starts_with("<body") {
                    inside_body = true;
                } else if tag_buffer.starts_with("</body>") {
                    inside_body = false;
                }

                // Write the tag buffer to the output
                writer.write_all(tag_buffer.as_bytes())?;
                tag_buffer.clear();
            } else if inside_tag {
                tag_buffer.push(ch);
            } else if inside_body {
                text_buffer.push(ch);
            } else {
                writer.write_all(ch.to_string().as_bytes())?;
            }
        }
    }

    if !text_buffer.is_empty() && inside_body {
        let processed_text = process_text_fn(&text_buffer);
        writer.write_all(processed_text.as_bytes())?;
    }

    writer.flush()?;
    Ok(())
}

// pub fn read_html_content(path: &str) -> Result<String, String> {
//     std::fs::read_to_string(path).map_err(|err| err.to_string())
// }

// pub fn write_html_content(path: &str, contents: &str) -> Result<(), String> {
//     std::fs::write(path, contents).map_err(|err| err.to_string())
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use tauri::Wry;
    #[test]
    fn test_process_html() {
        let process_text = Box::new(move |input: &str| {
            if input.trim().is_empty() {
                return input.to_string();
            }

            input.replace("world", "xiaoxiao").replace("fear", "fare")
        });
        let data = [
            (
                r#"<!DOCTYPE html><html lang="en"><head><meta charset="UTF-8"><title>hello world</title></head><body><div>hello <span style="color:red">world</span><img src="title.jpg"></div></body></html>"#,
                r#"<!DOCTYPE html><html lang="en"><head><meta charset="UTF-8"><title>hello world</title></head><body><div>hello <span style="color:red">xiaoxiao</span><img src="title.jpg"></div></body></html>"#,
            ),
            (
                r#"<html><head></head><body class="calibre"><p class="calibre_1">for <span style="color:red">fear</span> I</p></body></html>"#,
                r#"<html><head></head><body class="calibre"><p class="calibre_1">for <span style="color:red">fare</span> I</p></body></html>"#,
            ),
        ];

        for (input, expected) in data {
            let mut reader = Cursor::new(input);
            let mut writer = Cursor::new(Vec::new());
            let reporter: Option<&ProgressReporter<Wry>> = None;
            process_html(&mut reader, &mut writer, process_text.as_ref(), reporter).unwrap();
            let output_data = String::from_utf8(writer.into_inner()).unwrap();
            assert_eq!(output_data, expected);
        }
    }
}
