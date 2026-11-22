#![windows_subsystem = "windows"]

use evo75_keylab::{Collection, scan_receiver};

#[tauri::command]
async fn scan() -> Result<Vec<Collection>, String> {
    tauri::async_runtime::spawn_blocking(scan_receiver)
        .await
        .map_err(|error| error.to_string())?
        .map_err(|error| error.to_string())
}

#[tauri::command]
fn open_configurator() -> Result<(), String> {
    open::that("https://cfg.ticktype.com/").map_err(|error| error.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![scan, open_configurator])
        .run(tauri::generate_context!())
        .expect("Le démarrage de KeyLab a échoué");
}
