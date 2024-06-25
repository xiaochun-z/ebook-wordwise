mod annotation;
pub mod cmd;
pub mod html;
pub mod types;
use annotation::{load_dict, load_lemma};
use html::process_html;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use tauri::Runtime;
use types::{ChunkParameter, ProgressReporter, WorkMesg};

pub fn process<R: Runtime>(
    file: &str,
    language: &str,
    book_format: &str,
    include_phoneme: bool,
    def_len: i32,
    hint_level: i32,
    reporter: Option<&ProgressReporter<R>>,
) -> Result<(), String> {
    //println!("book format: {}", book_format);
    let lemma = load_lemma().map_err(|err| format!("lemmatization: {}", err))?;
    let dict = load_dict(language).map_err(|err| format!("dictionary-{}: {}", language, err))?;
    let param: ChunkParameter = ChunkParameter {
        format: book_format,
        dict: &dict,
        lemma: &lemma,
        def_length: def_len,
        including_phoneme: include_phoneme,
        hint_level,
    };

    let f = Path::new(file);
    let out_file = f.file_stem().unwrap().to_str().unwrap();
    let out_file_ext = f.extension().unwrap().to_str().unwrap();
    let out_file = format!(
        "{}\\{}.out.{}",
        f.parent().unwrap().to_str().unwrap(),
        out_file,
        out_file_ext
    );

    let input = File::open(file).unwrap();
    let mut reader = BufReader::new(input);
    let output = File::create(&out_file).unwrap();
    let mut writer = BufWriter::new(output);
    if let Some(reporter) = reporter {
        reporter
            .tauri_window
            .emit(
                "event-workmesg",
                WorkMesg::new("text-green-800 dark:text-green-300", "processing book..."),
            )
            .map_err(|e| e.to_string())?;
    }

    process_html(
        &mut reader,
        &mut writer,
        &param,
        html::process_text_fn,
        reporter,
    )
    .map_err(|err| err.to_string())?;

    // remove the source file
    std::fs::remove_file(file).map_err(|err| err.to_string())?;
    // replace the source file with new file
    std::fs::rename((&out_file).as_str(), file).map_err(|err| err.to_string())?;

    Ok(())
}
