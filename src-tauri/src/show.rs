use std::fs::File;

use tauri::{AppHandle, Manager};
use tauri_plugin_dialog::DialogExt;
use tauri_specta::Event;

use crate::{
    cue::{Cue, CueExecutionContext, CueId, CueList},
    mix::MixConfig,
    AppData, MutableState,
};

#[derive(Clone, Default, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ShowState {
    pub current_cue_id: Option<CueId>,
}

impl ShowState {
    fn reset(&mut self, handle: &AppHandle) {
        *self = Self::default();
        let _ = ShowStateEvent::Update(self.clone()).emit(handle);
    }

    fn update_current_cue(&mut self, cue_id: Option<CueId>, handle: &AppHandle) {
        self.current_cue_id = cue_id;
        let _ = ShowStateEvent::Update(self.clone()).emit(handle);
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, specta::Type, tauri_specta::Event)]
pub enum ShowStateEvent {
    Update(ShowState),
}

#[tauri::command]
#[specta::specta]
pub async fn goto_cue(
    handle: AppHandle,
    state: MutableState<'_, AppData>,
    cue_id: CueId,
) -> Result<(), String> {
    let mut app_data = state.write().await;

    let Some(console) = app_data.console.as_ref() else {
        return Err("Console not connected".to_string());
    };

    let Some(cue) = app_data.show.cues.get(&cue_id) else {
        return Err("Cue not found".to_string());
    };

    cue.activate(CueExecutionContext {
        config: &app_data.show.mix_config,
        wing: console,
    })
    .await
    .map_err(|err| format!("Failed to activate cue: {}", err))?;

    app_data
        .show_state
        .update_current_cue(Some(cue_id), &handle);

    Ok(())
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Show {
    pub mix_config: MixConfig,
    pub cues: CueList,
}

#[derive(Clone, serde::Serialize, serde::Deserialize, specta::Type, tauri_specta::Event)]
pub enum ShowEvent {
    Loaded(Show),
    CueAdded((u32, Cue)),
}

#[tauri::command]
#[specta::specta]
pub async fn get_show(state: MutableState<'_, AppData>) -> Result<(Show, ShowState), String> {
    let (show, state) = {
        let state = state.read().await;
        (state.show.clone(), state.show_state.clone())
    };

    Ok((show, state))
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
    app_data.show_state.reset(&handle);

    let _ = ShowEvent::Loaded(show)
        .emit(&handle)
        .inspect_err(|err| println!("Failed to send showfile load event: {}", err));

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn add_cue(handle: AppHandle, state: MutableState<'_, AppData>) -> Result<(), String> {
    let max_cue_id = {
        let app_data = state.read().await;
        app_data.show.cues.iter().map(|cue| cue.id).max()
    };

    let cue_id = max_cue_id
        .map(|id| id.next())
        .unwrap_or_else(|| CueId::new(1, 0));

    let name = format!("Cue {}", cue_id);

    let cue = Cue::new(cue_id, name);

    let cue_idx = {
        let mut app_data = state.write().await;
        app_data.show.cues.push(cue.clone())
    };

    let _ = ShowEvent::CueAdded((cue_idx as u32, cue)).emit(&handle);

    Ok(())
}
