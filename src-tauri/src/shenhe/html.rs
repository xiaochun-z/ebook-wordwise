use super::annotation::annotate_phrase;
use super::types::{ChunkParameter, ProcessChunkFn, ProgressReporter};
use rayon::prelude::*;
use std::io::{Read, Seek, SeekFrom, Write};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use tauri::Runtime;

pub fn process_html<R: Read + Seek, W: Write, Rt: Runtime>(
    reader: &mut R,
    writer: &mut W,
    param: &ChunkParameter,
    process_fn: ProcessChunkFn,
    reporter: Option<&ProgressReporter<Rt>>,
) -> Result<(), String> {
    const CHUNK_SIZE: usize = 100 * 1024; // 100 KB
    let chunks = split_html(reader, CHUNK_SIZE, param, process_fn, reporter)
        .map_err(|err| err.to_string())?;
    // let num_chunks = chunks.len();

    // let progress_counter = Arc::new(AtomicUsize::new(0));

    // // Process the chunks in parallel
    // let processed_chunks: Vec<String> = (&chunks)
    //     .into_par_iter()
    //     .map(|chunk| {
    //         let processed_chunk = process_fn(chunk.as_str(), param);
    //         let progress = progress_counter.fetch_add(1, Ordering::SeqCst) + 1;

    //         if let Some(reporter) = reporter {
    //             let prog = progress as f32 / num_chunks as f32;
    //             reporter.report(prog);
    //         }
    //         processed_chunk
    //     })
    //     .collect();

    // Write the processed chunks back in order
    for chunk in chunks {
        writer
            .write(chunk.as_bytes())
            .map_err(|err| err.to_string())?;
    }

    Ok(())
}

pub fn process_text_fn(input: &str, param: &ChunkParameter) -> String {
    if input.trim().is_empty() {
        return input.to_string();
    }

    let res = annotate_phrase(
        param.annotator,
        input,
        param.dict,
        param.lemma,
        param.def_length,
    );
    res
}

// fn find_valid_utf8_boundary(buffer: &[u8], end: usize) -> usize {
//     let mut valid_end = end;

//     // Safely convert the slice to a string
//     while valid_end > 0 {
//         if let Ok(buffer_str) = std::str::from_utf8(&buffer[..valid_end]) {
//             if buffer_str.is_char_boundary(valid_end) {
//                 break;
//             }
//         }
//         valid_end -= 1;
//     }

//     valid_end
// }
fn split_html<R: Read + Seek, Rt: Runtime>(
    reader: &mut R,
    max_size: usize,
    param: &ChunkParameter,
    process_fn: ProcessChunkFn,
    reporter: Option<&ProgressReporter<Rt>>,
) -> Result<Vec<String>, String> {
    let file_size = reader
        .seek(SeekFrom::End(0))
        .map_err(|err| err.to_string())? as usize;
    reader
        .seek(SeekFrom::Start(0))
        .map_err(|err| err.to_string())?;
    let mut buffer = vec![0; file_size as usize];
    reader
        .read_exact(&mut buffer)
        .map_err(|err| format!("{:?}", err))?;

    let body_position = buffer
        .windows("<body".len())
        .position(|x| x == b"<body")
        .unwrap_or(0);

    let body_position = body_position
        + buffer[body_position..]
            .windows(">".len())
            .position(|x| x == b">")
            .unwrap_or(0)
        + ">".len();

    let body_end_position = body_position
        + buffer[body_position..]
            .windows("</body>".len())
            .position(|x| x == b"</body>")
            .unwrap_or(0);

    let mut chunks = Vec::new();

    if body_position > 0 {
        chunks.push(String::from_utf8_lossy(&buffer[..body_position]).into_owned());
    }

    if body_position < file_size as usize {
        if body_end_position < file_size as usize {
            let body_chunks = split_chunk(&buffer[body_position..body_end_position], max_size);
            let num_chunks = body_chunks.len();

            let progress_counter = Arc::new(AtomicUsize::new(0));
            let new_chunks: Vec<String> = body_chunks
                .par_iter()
                .map(|x| {
                    let progress = progress_counter.fetch_add(1, Ordering::SeqCst) + 1;

                    if let Some(reporter) = reporter {
                        let prog = progress as f32 / num_chunks as f32;
                        reporter.report(prog);
                    }
                    process_text(x, param, process_fn)
                })
                .collect();
            chunks.extend(new_chunks);

            chunks.push(String::from_utf8_lossy(&buffer[body_end_position..]).into_owned());
        } else {
            chunks.push(String::from_utf8_lossy(&buffer[body_position..]).into_owned());
        }
    }

    Ok(chunks)
}

// when splitting the html try to break the chunks by html tag, we keep the html tag in the same chunk, for example "<span> this is a test sentence></span><span> this is another test sentence></span>" will be split into ["<span> this is a test sentence</span>","<span> this is another test sentence></span>"]
// we don't make it something like ["<span> this is a test sentence</s","pan><span> this is another test sentence></span>"]
// even after contenate the chunks, they produce the same result, but the later one break html tag, it can leads missunderstanding,
// we keep the html tag in the same chunk, if the text content is too long and make one chunk too huge, we still keep the huge chunk, we don't break it.
// parameter: file - the html file name
// parameter: max_size - the max size of each chunk
// return: vec<String> - the list of chunks
fn split_chunk(buffer: &[u8], max_size: usize) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut start = 0;
    let buffer_len = buffer.len();
    let tags = ["</p>", "</span>", "</div>", " "];

    while start < buffer_len {
        let end = (start + max_size).min(buffer_len);
        if end == 0 {
            break;
        }

        let mut found_tag = false;
        for &tag in &tags {
            if let Some(chunk) = shrink_chunk_to_match_html_tag(&buffer[start..end], tag) {
                let len = chunk.len();
                chunks.push(std::str::from_utf8(chunk).unwrap().to_string());
                start += len;
                found_tag = true;
                break;
            }
        }

        if !found_tag {
            chunks.push(
                std::str::from_utf8(&buffer[start..end])
                    .unwrap()
                    .to_string(),
            );
            start = end;
        }
    }

    chunks
}

pub fn process_text(html: &str, param: &ChunkParameter, process_fn: ProcessChunkFn) -> String {
    if html.is_empty() {
        return html.to_string();
    }

    if !html.contains('<') {
        return process_fn(html, &param);
    }

    let mut all = String::with_capacity(html.len()); // Pre-allocate the string with the input length
    let mut text = String::new();
    let mut in_tag = false;

    for ch in html.chars() {
        match ch {
            '>' => {
                in_tag = false;
                all.push(ch);
            }
            '<' => {
                in_tag = true;

                if !text.is_empty() {
                    all.push_str(&process_fn(&text, &param));
                    text.clear();
                }

                all.push(ch);
            }
            _ => {
                if in_tag {
                    all.push(ch);
                } else {
                    text.push(ch);
                }
            }
        }
    }

    if !text.is_empty() {
        all.push_str(&process_fn(&text, &param));
    }

    all
}

fn shrink_chunk_to_match_html_tag<'a>(buffer: &'a [u8], tag: &str) -> Option<&'a [u8]> {
    let tag_bytes = tag.as_bytes();
    let mut end = buffer.len();

    while end >= tag_bytes.len() {
        // Ensure valid UTF-8 boundary by moving backwards to the start of a valid UTF-8 sequence
        while end > 0 && !std::str::from_utf8(&buffer[..end]).is_ok() {
            end -= 1;
        }

        if chunk_end_with(&buffer[..end], tag) {
            return Some(&buffer[..end]);
        }

        end -= 1;
    }

    None
}

fn chunk_end_with(chunk: &[u8], str: &str) -> bool {
    let strbytes = str.as_bytes();
    chunk.ends_with(strbytes)
}

#[cfg(test)]
mod tests {
    use super::super::types::{Annotator, DictRecord, ProgressReporter};
    use super::{process_html, ChunkParameter};
    use std::collections::HashMap;
    use std::io::Cursor;
    use tauri::Wry;

    fn fake_process_text(input: &str, _param: &ChunkParameter) -> String {
        if input.trim().is_empty() {
            return input.to_string();
        }

        input.replace("world", "xiaoxiao").replace("fear", "fare")
    }

    #[test]
    fn test_process_html() {
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
        let mut lemma = HashMap::new();
        lemma.insert("world".to_string(), "world".to_string());

        let mut dict = HashMap::new();
        let dr = DictRecord {
            word: "world".to_string(),
            phoneme: "".to_string(),
            full_def: "xiaoxiao".to_string(),
            short_def: "xiaoxiao".to_string(),
            example_sentences: "".to_string(),
            hint_lvl: 3,
        };
        dict.insert("world".to_string(), dr);

        let dr = DictRecord {
            word: "fear".to_string(),
            phoneme: "".to_string(),
            full_def: "fare".to_string(),
            short_def: "fare".to_string(),
            example_sentences: "".to_string(),
            hint_lvl: 3,
        };
        dict.insert("world".to_string(), dr);
        let annotator = Annotator::InlineAnnotator(3, false);
        let param: ChunkParameter = ChunkParameter {
            dict: &dict,
            lemma: &lemma,
            def_length: 1,
            annotator: &annotator,
        };

        for (input, expected) in data {
            let mut reader = Cursor::new(input);
            let mut writer = Cursor::new(Vec::new());
            let reporter: Option<&ProgressReporter<Wry>> = None;
            process_html(
                &mut reader,
                &mut writer,
                &param,
                fake_process_text,
                reporter,
            )
            .unwrap();
            let vec_w = writer.into_inner();
            let output_data = String::from_utf8(vec_w).unwrap();
            assert_eq!(output_data, expected);
        }
    }
}
