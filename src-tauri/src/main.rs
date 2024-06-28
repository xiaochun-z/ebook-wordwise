// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serde::Deserialize;
//use serde_json::from_str;
//use tauri::AppHandle;
//use tauri::{Event, Manager, Runtime};
mod shenhe;
use shenhe::{
    annotation::{load_dict, load_lemma},
    cmd::{ebook_convert_exists, run_command},
    html::{self, process_text},
    process,
    types::{Annotator, ChunkParameter, Payload, ProgressReporter, WorkMesg},
};
use std::path::Path;
use tauri::{api::dialog::FileDialogBuilder, Builder, Runtime};
use uuid::Uuid;

// fn get_path() -> Result<std::path::PathBuf, String> {
//     let cwd = std::env::current_dir().map_err(|e| e.to_string())?;
//     Ok(cwd.join("settings.json"))
// }

// #[tauri::command]
// async fn save_settings<R: Runtime>(
//     _app: tauri::AppHandle<R>,
//     _window: tauri::Window<R>,
//     settings: AppSetting,
// ) -> Result<(), String> {
//     let json = serde_json::to_string(&settings).unwrap();
//     println!("{}", &json);
//     // get current directory and concat with settings.json
//     let path = get_path()?;
//     std::fs::write(path, json).map_err(|e| e.to_string())?;
//     // print to console
//     Ok(())
// }

// #[tauri::command]
// async fn read_settings<R: Runtime>(
//     _app: tauri::AppHandle<R>,
//     window: tauri::Window<R>,
// ) -> Result<(), String> {
//     // read settings.json
//     println!("calling read_settings");
//     let path = get_path()?;
//     let json = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
//     println!("{}", &json);
//     let settings: AppSetting = serde_json::from_str(&json).map_err(|e| e.to_string())?;
//     // return settings object to javascript app
//     window
//         .emit("settings_retrived", settings)
//         .map_err(|e| e.to_string())?;
//     Ok(())
// }

#[tauri::command]
async fn open_file_dialog(initial_path: String) -> Result<String, String> {
    //println!("{}", initial_path);
    let (tx, rx) = tokio::sync::oneshot::channel();

    let initial_path = Path::new(&initial_path);
    let dialog = if initial_path.exists() {
        FileDialogBuilder::new().set_directory(initial_path)
    } else {
        FileDialogBuilder::new()
    };

    dialog.pick_file(move |file_path| {
        if let Some(path) = file_path {
            //println!("Selected file: {:?}", path);
            let res = path.to_string_lossy().into_owned();
            tx.send(Ok(res)).unwrap();
        } else {
            tx.send(Err("No file selected".into())).unwrap();
        }
    });
    rx.await.unwrap()
}

fn progress_fn<R: Runtime>(progress: f32, tauri_window: &tauri::Window<R>) {
    let percent = (0.2 + (0.9 - 0.2) * progress) * 100.0; // map to [20%, 90%]
    tauri_window.emit("event-progress", percent).unwrap();
}

// fn button_click_handler(event: Event) {
//     // convert payload to struct Payload
//     let payload: Payload = from_str(event.payload().unwrap()).unwrap();
//     println!("payload: {:?}", payload);
// }
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
        format: &payload.format,
        dict: &dict,
        lemma: &lemma,
        def_length: def_len,
        including_phoneme: payload.show_phoneme,
        hint_level: payload.hint_level,
        annotator: &annotator,
    };

    process_text(original, &param, html::process_text_fn)
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
    std::fs::create_dir_all(&book_out_dir).map_err(|e| e.to_string())?;
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
        (&payload).format.as_str(),
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

fn main() {
    //use shenhe::{DictRow,load_dict};
    // use shenhe::html;
    // html::main();

    Builder::default()
        .invoke_handler(tauri::generate_handler![
            // read_settings,
            // save_settings,
            open_file_dialog,
            start_job,
            check_ebook_convert,
            preview,
        ])
        // .setup(|app| {
        //let main_window = app.get_window("main").unwrap();
        //main_window.listen("event-startjob", button_click_handler);
        // let handler_id = main_window.listen("event-startjob", button_click_handler);
        //main_window.unlisten(handler_id);
        //     Ok(())
        // })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(serde::Serialize, Deserialize, Clone, Debug)]
struct AppSetting {
    theme: String,
}
