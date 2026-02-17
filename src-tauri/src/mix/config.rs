use std::collections::HashMap;

use tauri::AppHandle;
use tauri_specta::Event;

use crate::{
    mix::{error::MixError, Actor, ActorEvent, ActorId},
    wing::{WingChannelId, WingColor},
    AppData, MutableState,
};

#[derive(Clone, Default, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct MixConfig {
    actors: HashMap<ActorId, Actor>,
}

impl MixConfig {
    pub fn actor(&self, id: ActorId) -> Result<&Actor, MixError> {
        self.actors.get(&id).ok_or(MixError::ActorNotFound(id))
    }

    pub fn controlled_channels<'a>(&'a self) -> impl Iterator<Item = &'a WingChannelId> {
        self.actors.values().map(|actor| actor.channel())
    }
}

impl MixConfig {
    pub fn add_actor(
        &mut self,
        channel: WingChannelId,
        name: String,
        color: Option<WingColor>,
    ) -> (ActorId, Actor) {
        let id: ActorId = self
            .actors
            .keys()
            .max()
            .map(|id| id.next())
            .unwrap_or_default();
        let actor = Actor::new(channel, name, color);

        self.actors.insert(id, actor.clone());
        (id, actor)
    }
}

#[tauri::command]
#[specta::specta]
pub async fn add_actor(
    handle: AppHandle,
    state: MutableState<'_, AppData>,
    channel: WingChannelId,
    name: String,
    color: Option<WingColor>,
) -> Result<ActorId, String> {
    let mut app_data = state.write().await;

    let (id, actor) = app_data.show.mix_config.add_actor(channel, name, color);
    let _ = ActorEvent::Added(id, actor)
        .emit(&handle)
        .inspect_err(|err| println!("Failed to emit actor added event: {}", err));

    Ok(id)
}
