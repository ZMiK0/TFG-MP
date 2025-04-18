use std::path::PathBuf;
use logic::add_playlist;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri_plugin_fs::FsExt;

mod logic;
use logic::Playlist;
use logic::Song;

#[tauri::command(rename_all = "snake_case")]
async fn sync_lib(music_dir:String, app_data_dir:String) {
    let _ = logic::sync(PathBuf::from(app_data_dir),PathBuf::from(music_dir));
}


#[tauri::command(rename_all = "snake_case")]
async fn get_all_playlists(db_path:String) -> Result<Vec<Playlist>, String> {
    let playlists = logic::get_all_playlists(db_path)?;

    Ok(playlists)
}

#[tauri::command(rename_all = "snake_case")]
async fn get_all_songs(db_path:String) -> Result<Vec<Song>, String> {
    let all_songs = logic::get_all_songs(db_path)?;

    Ok(all_songs)
}

#[tauri::command(rename_all = "snake_case")]
async fn create_playlist(name:String, cover_path:String, db_path:String) {
    let _ = add_playlist(name, cover_path, db_path);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let scope = app.fs_scope();
            if let Err(e) = scope.allow_directory("$HOME/Music", true) {
                eprintln!("Failed access: {}", e)
            }
            if let Err(e) = scope.allow_directory("$APPDATA", true) {
                eprintln!("Failed access: {}", e)
            }
            if let Err(e) = scope.allow_directory("$HOME/.config", true) {
                eprintln!("Failed access: {}", e)
            }
            if let Err(e) = scope.allow_directory("$HOME/Library/Application Support", true) {
                eprintln!("Failed access: {}", e)
            }
            if let Err(e) = scope.allow_directory("src-tauri", true) {
                eprintln!("Failed access: {}", e)
            }
            
            Ok(())
         })
        .invoke_handler(tauri::generate_handler![sync_lib,get_all_playlists,get_all_songs,create_playlist])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
