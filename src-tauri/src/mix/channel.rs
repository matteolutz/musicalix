use crate::{
    wing::{WingChannelId, WingChannelInfo, WingConsoleExt},
    AppData, MutableState,
};

#[tauri::command]
#[specta::specta]
pub async fn get_wing_channel_info(
    state: MutableState<'_, AppData>,
    channel: u8,
) -> Result<WingChannelInfo, String> {
    let mut app_data = state.write().await;
    let Some(wing) = app_data.console.as_mut() else {
        return Err("Console not connected".to_string());
    };

    let channel_id: WingChannelId = channel
        .try_into()
        .map_err(|err| format!("Invalid channel ID: {}", err))?;

    let mut channel = wing.channel(channel_id);
    channel
        .get_info()
        .map_err(|err| format!("Failed to get channel info: {}", err))
}
