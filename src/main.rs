#![windows_subsystem = "windows"]

mod via;

use evo75_keylab::{Collection, scan_receiver};

#[tauri::command]
async fn scan() -> Result<Vec<Collection>, String> {
    tauri::async_runtime::spawn_blocking(scan_receiver)
        .await
        .map_err(|error| error.to_string())?
        .map_err(|error| error.to_string())
}

#[tauri::command]
async fn query_via(color: Option<[u8; 2]>) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || via::communicate(color))
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
fn open_configurator() -> Result<(), String> {
    open::that("https://cfg.ticktype.com/").map_err(|error| error.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![scan, query_via, open_configurator])
        .run(tauri::generate_context!())
        .expect("Le démarrage de KeyLab a échoué");
}
