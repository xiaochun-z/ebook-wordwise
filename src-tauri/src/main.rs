// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serde::Deserialize;
use tauri::Runtime;
mod shenhe;

fn get_path() -> Result<std::path::PathBuf, String> {
    let cwd = std::env::current_dir().map_err(|e| e.to_string())?;
    Ok(cwd.join("settings.json"))
}

#[tauri::command]
async fn save_settings<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
    settings: AppSetting,
) -> Result<(), String> {
    let json = serde_json::to_string(&settings).unwrap();
    println!("{}", &json);
    // get current directory and concat with settings.json
    let path = get_path()?;
    std::fs::write(path, json).map_err(|e| e.to_string())?;
    // print to console
    Ok(())
}

#[tauri::command]
async fn read_settings<R: Runtime>(
    _app: tauri::AppHandle<R>,
    window: tauri::Window<R>,
) -> Result<(), String> {
    // read settings.json
    println!("calling read_settings");
    let path = get_path()?;
    let json = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    println!("{}", &json);
    let settings: AppSetting = serde_json::from_str(&json).map_err(|e| e.to_string())?;
    // return settings object to javascript app
    window
        .emit("settings_retrived", settings)
        .map_err(|e| e.to_string())?;
    Ok(())
}

fn main() {
    //use shenhe::{DictRow,load_dict};
    // use shenhe::html;
    // html::main();
    use shenhe::process;
    process("resources/sample.xml","en");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read_settings, save_settings])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(serde::Serialize, Deserialize, Clone, Debug)]
struct AppSetting {
    theme: String,
}
