use std::collections::BTreeSet;

use crate::{mix::ActorId, wing::WingColor};

#[repr(transparent)]
#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    specta::Type,
)]
pub struct GroupId(u32);

impl GroupId {
    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

impl From<u32> for GroupId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for GroupId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    name: String,
    actors: BTreeSet<ActorId>,
    color: Option<WingColor>,
}

impl Group {
    pub fn new(
        actors: impl IntoIterator<Item = ActorId>,
        name: String,
        color: Option<WingColor>,
    ) -> Self {
        Self {
            name,
            actors: actors.into_iter().collect(),
            color,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn actors(&self) -> impl Iterator<Item = &ActorId> {
        self.actors.iter()
    }

    pub fn color(&self) -> Option<WingColor> {
        self.color
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize, specta::Type, tauri_specta::Event)]
pub enum GroupEvent {
    Added(GroupId, Group),
    Removed(GroupId),
}
