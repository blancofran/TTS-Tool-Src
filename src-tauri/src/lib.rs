mod audio;
mod commands;
mod error;
mod models;
mod state;
mod whisper;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(state::TranscriptionState::default())
        .invoke_handler(tauri::generate_handler![
            commands::transcribe::transcribe_file,
            commands::model::get_model_status,
            commands::model::download_model,
            commands::model::cancel_transcription,
            commands::export::export_transcript,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
