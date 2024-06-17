mod annotation;
pub mod cmd;
pub mod html;
pub mod types;

use annotation::{annotate_phrase, load_dict, load_lemma};
use html::{process_html, read_html_content};
use tauri::Runtime;
use types::RubyAnnotator;

pub fn process<R: Runtime>(
    file: &str,
    language: &str,
    book_format: &str,
    include_phoneme: bool,
    def_len: i32,
    hint_level: i32,
    progress_fn: &(dyn Fn(f32, Option<&tauri::Window<R>>)),
    tauri_window: Option<&tauri::Window<R>>,
) -> Result<String, String> {
    println!("book format: {}", book_format);
    let lemma = load_lemma().unwrap();
    let annotation_dict = load_dict(language).unwrap();
    let ruby_annotator = RubyAnnotator {};

    let process_text_wrapper = Box::new(move |input: &str| {
        if input.trim().is_empty() {
            return input.to_string();
        }

        let res = annotate_phrase(
            &ruby_annotator,
            input,
            &annotation_dict,
            &lemma,
            def_len,
            include_phoneme,
            hint_level,
        );
        res
    });

    let fn_ptr: &dyn Fn(&str) -> String = process_text_wrapper.as_ref();

    let html_content = read_html_content(file)?;

    let html_content = html_content;
    let new_html_content = process_html(
        html_content.as_str(),
        fn_ptr,
        Some(&progress_fn),
        tauri_window,
    )?;
    //println!("new_html_content: {}", new_html_content);
    //let file_name_without_path = file.split('/').last().unwrap();
    //Ok(format!("{} was processed successfully",  file_name_without_path))
    Ok(new_html_content)
}
