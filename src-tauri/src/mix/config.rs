use std::collections::HashMap;

use crate::{
    mix::{error::MixError, Actor, ActorId},
    wing::WingChannelId,
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
