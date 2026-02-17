use std::fs::File;

use tauri::{AppHandle, Manager};
use tauri_plugin_dialog::DialogExt;
use tauri_specta::Event;

use crate::{cue::Cue, mix::MixConfig, AppData, MutableState};

#[derive(Clone, Default, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Show {
    pub mix_config: MixConfig,
    pub cues: Vec<Cue>,
}

#[derive(Clone, serde::Serialize, serde::Deserialize, specta::Type, tauri_specta::Event)]
pub enum ShowEvent {
    Loaded(Show),
}

#[tauri::command]
#[specta::specta]
pub async fn open_showfile(
    handle: AppHandle,
    state: MutableState<'_, AppData>,
    file_path: String,
) -> Result<(), String> {
    let show_file = File::open(file_path).map_err(|err| format!("Failed to open file: {}", err))?;
    let show: Show = serde_json::from_reader(&show_file)
        .map_err(|err| format!("Failed to parse show file: {}", err))?;

    {
        let mut app_state = state.write().await;
        app_state.show = show.clone();
    }

    let _ = ShowEvent::Loaded(show).emit(&handle);

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn get_show(state: MutableState<'_, AppData>) -> Result<Show, String> {
    let show = state.read().await.show.clone();
    Ok(show)
}

pub async fn save_show_as(handle: AppHandle) -> Result<(), String> {
    let save_file_path = tauri::async_runtime::spawn_blocking({
        let handle = handle.clone();
        move || {
            handle
                .clone()
                .dialog()
                .file()
                .add_filter("musicalix Show-File", &["json"])
                .blocking_save_file()
        }
    })
    .await
    .map_err(|err| format!("Failed to save file: {}", err))?
    .ok_or_else(|| format!("Failed to save file"))?
    .into_path()
    .map_err(|err| format!("Failed to convert to path: {}", err))?;

    let file =
        File::create(&save_file_path).map_err(|err| format!("Failed to open file: {}", err))?;

    {
        let app_data: MutableState<'_, AppData> = handle.state();
        let app_data = app_data.read().await;

        #[cfg(debug_assertions)]
        serde_json::to_writer_pretty(&file, &app_data.show)
            .map_err(|err| format!("Fialed to write showfile: {}", err))?;

        #[cfg(not(debug_assertions))]
        serde_json::to_writer(&file, &app_data.show)
            .map_err(|err| format!("Fialed to write showfile: {}", err))?;
    }

    let app_data: MutableState<'_, AppData> = handle.state();
    let mut app_data = app_data.write().await;
    let _ = handle
        .get_webview_window("main")
        .unwrap()
        .set_title(save_file_path.display().to_string().as_str());
    app_data.current_show_file_path = Some(save_file_path);

    Ok(())
}

pub async fn save_show(handle: AppHandle) -> Result<(), String> {
    let current_file_path = {
        let app_data: MutableState<'_, AppData> = handle.state();
        let app_data = app_data.read().await;
        app_data.current_show_file_path.clone()
    };

    let Some(current_file_path) = current_file_path else {
        return save_show_as(handle).await;
    };

    let file =
        File::create(current_file_path).map_err(|err| format!("Failed to open file: {}", err))?;

    let app_data: MutableState<'_, AppData> = handle.state();
    let app_data = app_data.read().await;

    #[cfg(debug_assertions)]
    return serde_json::to_writer_pretty(&file, &app_data.show)
        .map_err(|err| format!("Fialed to write showfile: {}", err));

    #[cfg(not(debug_assertions))]
    return serde_json::to_writer(&file, &app_data.show)
        .map_err(|err| format!("Fialed to write showfile: {}", err));
}

pub async fn open_show(handle: AppHandle) -> Result<(), String> {
    let open_file_path = tauri::async_runtime::spawn_blocking({
        let handle = handle.clone();
        move || {
            handle
                .clone()
                .dialog()
                .file()
                .add_filter("musicalix Show-File", &["json"])
                .blocking_pick_file()
        }
    })
    .await
    .map_err(|err| format!("Failed to open file: {}", err))?
    .ok_or_else(|| format!("Failed to open file"))?
    .into_path()
    .map_err(|err| format!("Failed to convert to path: {}", err))?;

    let file =
        File::open(&open_file_path).map_err(|err| format!("Failed to open file: {}", err))?;

    let show: Show = serde_json::from_reader(file)
        .map_err(|err| format!("Failed to parse show file: {}", err))?;

    let app_data: MutableState<'_, AppData> = handle.state();
    let mut app_data = app_data.write().await;
    app_data.show = show.clone();
    let _ = handle
        .get_webview_window("main")
        .unwrap()
        .set_title(open_file_path.display().to_string().as_str());
    app_data.current_show_file_path = Some(open_file_path);

    let _ = ShowEvent::Loaded(show)
        .emit(&handle)
        .inspect_err(|err| println!("Failed to send showfile load event: {}", err));

    Ok(())
}
