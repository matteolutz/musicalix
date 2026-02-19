use std::collections::HashMap;

use tauri::AppHandle;
use tauri_specta::Event;

use crate::{
    mix::{
        error::MixError, Actor, ActorEvent, ActorId, Group, GroupEvent, GroupId, Position,
        PositionId,
    },
    wing::{id::WingId, WingChannelId, WingColor},
    AppData, MutableState,
};

#[derive(Clone, Default, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct MixConfig {
    actors: HashMap<ActorId, Actor>,
    groups: HashMap<GroupId, Group>,
    positions: HashMap<PositionId, Position>,
}

impl MixConfig {
    pub fn actors(&self) -> impl Iterator<Item = (&ActorId, &Actor)> {
        self.actors.iter()
    }

    pub fn actor(&self, id: ActorId) -> Result<&Actor, MixError> {
        self.actors.get(&id).ok_or(MixError::ActorNotFound(id))
    }

    pub fn groups(&self) -> impl Iterator<Item = (&GroupId, &Group)> {
        self.groups.iter()
    }

    pub fn group(&self, id: GroupId) -> Result<&Group, MixError> {
        self.groups.get(&id).ok_or(MixError::GroupNotFound(id))
    }

    pub fn positions(&self) -> impl Iterator<Item = (&PositionId, &Position)> {
        self.positions.iter()
    }

    pub fn position(&self, id: PositionId) -> Result<&Position, MixError> {
        self.positions
            .get(&id)
            .ok_or(MixError::PositionNotFound(id))
    }

    pub fn controlled_channels<'a>(&'a self) -> impl Iterator<Item = &'a WingChannelId> {
        self.actors.values().map(|actor| actor.channel())
    }
}

impl MixConfig {
    pub fn insert_actor(&mut self, actor: Actor) -> ActorId {
        let id: ActorId = self
            .actors
            .keys()
            .max()
            .map(|id| id.next())
            .unwrap_or_default();

        self.actors.insert(id, actor.clone());
        id
    }

    pub fn add_actor(
        &mut self,
        channel: WingChannelId,
        name: String,
        color: Option<WingColor>,
    ) -> (ActorId, Actor) {
        let actor = Actor::new(channel, name, color);
        let id = self.insert_actor(actor.clone());

        (id, actor)
    }

    pub fn add_group(
        &mut self,
        actors: Vec<ActorId>,
        name: String,
        color: Option<WingColor>,
    ) -> (GroupId, Group) {
        let id: GroupId = self
            .groups
            .keys()
            .max()
            .map(|id| id.next())
            .unwrap_or_default();
        let group = Group::new(actors, name, color);

        self.groups.insert(id, group.clone());
        (id, group)
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

#[tauri::command]
#[specta::specta]
pub async fn import_actors(
    handle: AppHandle,
    state: MutableState<'_, AppData>,
    from_channel: WingChannelId,
    to_channel: WingChannelId,
) -> Result<(), String> {
    let mut app_data = state.write().await;

    let Some(console) = app_data.console.as_ref() else {
        return Err("Console not connected".to_string());
    };

    let mut actors = Vec::new();
    for i in from_channel.value()..=to_channel.value() {
        let channel_id: WingChannelId = i.try_into().unwrap();
        let channel = console.channel(channel_id);
        let Some(channel_info) = channel.get_info().await.ok() else {
            continue;
        };

        actors.push(Actor::from_channel_info(channel_id, channel_info));
    }

    for actor in actors {
        let id = app_data.show.mix_config.insert_actor(actor.clone());

        let _ = ActorEvent::Added(id, actor)
            .emit(&handle)
            .inspect_err(|err| println!("Failed to emit actor added event: {}", err));
    }

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn add_group(
    handle: AppHandle,
    state: MutableState<'_, AppData>,
    actors: Vec<ActorId>,
    name: String,
    color: Option<WingColor>,
) -> Result<GroupId, String> {
    let mut app_data = state.write().await;

    let (id, group) = app_data.show.mix_config.add_group(actors, name, color);
    let _ = GroupEvent::Added(id, group)
        .emit(&handle)
        .inspect_err(|err| println!("Failed to emit group added event: {}", err));

    Ok(id)
}
