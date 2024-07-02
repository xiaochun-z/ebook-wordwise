// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serde::Deserialize;

mod shenhe;
use shenhe::{
    annotation::{load_dict, load_lemma},
    cmd::{ebook_convert_exists, run_command},
    html::{self, process_text},
    process,
    types::{Annotator, ChunkParameter, Payload, ProgressReporter, WorkMesg, APP_DATA_DIR},
};
use std::{error::Error, path::Path};
use tauri::api::path::resource_dir;
use tauri::{Builder, Manager, Runtime};
use uuid::Uuid;
const RESORUCE_FOLDER: &'static str = "resources";

fn progress_fn<R: Runtime>(progress: f32, tauri_window: &tauri::Window<R>) {
    let percent = (0.2 + (0.9 - 0.2) * progress) * 100.0; // map to [20%, 90%]
    tauri_window.emit("event-progress", percent).unwrap();
}

#[tauri::command]
fn check_ebook_convert() -> Result<bool, String> {
    let exists = ebook_convert_exists();
    Ok(exists)
}

#[tauri::command]
fn preview(payload: Payload, original: &str) -> String {
    //println!("payload: {:?}", payload);
    let lemma = load_lemma().unwrap();
    let dict = load_dict(payload.language.as_str()).unwrap();
    let annotator = match payload.wordwise_style {
        0 => Annotator::InlineAnnotator(payload.hint_level, payload.show_phoneme),
        1 => Annotator::RubyAnnotator(payload.hint_level, payload.show_phoneme),
        2 => Annotator::ColorAnnotator("red", payload.hint_level, payload.show_phoneme),
        _ => Annotator::InlineAnnotator(payload.hint_level, payload.show_phoneme),
    };

    let def_len = match payload.allow_long {
        false => 1,
        true => 2,
    };
    let param: ChunkParameter = ChunkParameter {
        dict: &dict,
        lemma: &lemma,
        def_length: def_len,
        annotator: &annotator,
    };

    process_text(original, &param, html::process_text_fn)
}

#[tauri::command]
async fn open_directory<R: Runtime>(app: tauri::AppHandle<R>) -> Result<(), String> {
    let env = app.env();
    if let Some(resource) = resource_dir(app.package_info(), &env) {
        let resource = resource.join(RESORUCE_FOLDER);
        let resource = resource.to_str().unwrap();

        let reporter: Option<&ProgressReporter<R>> = None;
        let os = std::env::consts::OS;
        match os {
            "windows" => run_command("explorer", reporter, &[resource])?,
            "macos" => run_command("open", reporter, &["-R", resource])?,
            "linux" => run_command("open", reporter, &[resource])?,
            _ => format!("Running on an unsupported operating system: {}", os),
        };
    }

    Ok(())
}

#[tauri::command]
async fn start_job<R: Runtime>(
    window: tauri::Window<R>,
    payload: Payload,
) -> Result<String, String> {
    window
        .emit("event-progress", 0.0)
        .map_err(|e| e.to_string())?;
    const EBOOK_CONVERT: &'static str = "ebook-convert";
    let book = (&payload).book.as_str();
    if book.is_empty() {
        return Err("Empty book path, please select a book.".to_string());
    }

    let book_path = Path::new(book).parent().unwrap().to_str().unwrap();
    let book_name_without_ext = Path::new(book).file_stem().unwrap().to_str().unwrap();

    let uuid = Uuid::new_v4().to_string();
    let book_out_dir: String = format!("{}/{}/", book_path, uuid);
    //std::fs::create_dir_all(&book_out_dir).map_err(|e| e.to_string())?;
    let book_dump = format!("{}/{}.htmlz", book_path, book_name_without_ext);
    let reporter = ProgressReporter::new(&window, progress_fn);

    window
        .emit(
            "event-workmesg",
            WorkMesg::new(
                "text-green-800 dark:text-green-300",
                r#"Awaiting Calibre's "ebook-convert" to convert ebook to HTML."#,
            ),
        )
        .map_err(|e| e.to_string())?;

    run_command(EBOOK_CONVERT, Some(&reporter), &[book, book_dump.as_str()])?;
    window
        .emit("event-progress", 10.0)
        .map_err(|e| e.to_string())?;
    run_command(
        EBOOK_CONVERT,
        Some(&reporter),
        &[book_dump.as_str(), (&book_out_dir).as_str()],
    )?;
    window
        .emit("event-progress", 20.0)
        .map_err(|e| e.to_string())?;

    let html_file = format!("{}/index1.html", book_out_dir);
    let artifact_file = format!(
        "{}/{}-wordwise.{}",
        book_path,
        book_name_without_ext,
        (&payload).format
    );

    process(
        html_file.as_str(),
        (&payload).language.as_str(),
        (&payload).show_phoneme,
        if (&payload).allow_long { 2 } else { 1 },
        (&payload).hint_level,
        (&payload).wordwise_style,
        Some(&reporter),
    )?;
    window
        .emit(
            "event-workmesg",
            WorkMesg::new(
                "text-green-800 dark:text-green-300",
                r#"Awaiting Calibre's "ebook-convert" to convert HTML back to ebook."#,
            ),
        )
        .map_err(|e| e.to_string())?;
    let meta_file = format!("{}/content.opf", book_out_dir);
    run_command(
        EBOOK_CONVERT,
        Some(&reporter),
        &[
            html_file.as_str(),
            artifact_file.as_str(),
            "-m",
            meta_file.as_str(),
        ],
    )?;
    // remove the temp files and folders
    std::fs::remove_file(book_dump).map_err(|e| e.to_string())?;
    std::fs::remove_dir_all(book_out_dir).map_err(|e| e.to_string())?;
    window
        .emit("event-progress", 100.0)
        .map_err(|e| e.to_string())?;
    let artifact_file = Path::new(artifact_file.as_str())
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();
    Ok(format!("{} save to {}", artifact_file, book_path))
}

fn setup_data(app: &mut tauri::App) -> Result<(), Box<dyn Error>> {
    // if let Some(data_dir) = data_dir() {
    let env = app.env();
    if let Some(resource) = resource_dir(app.package_info(), &env) {
        let app_resource = resource.join(RESORUCE_FOLDER);
        APP_DATA_DIR
            .set(app_resource.to_string_lossy().into_owned())
            .ok();
    }
    Ok(())
}
fn main() {
    Builder::default()
        .invoke_handler(tauri::generate_handler![
            start_job,
            check_ebook_convert,
            preview,
            open_directory,
        ])
        .setup(setup_data)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(serde::Serialize, Deserialize, Clone, Debug)]
struct AppSetting {
    theme: String,
}
