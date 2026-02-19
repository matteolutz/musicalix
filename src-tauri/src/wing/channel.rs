use std::{collections::HashSet, fmt::Debug};

use itertools::Itertools;
use libwing::WingConsole;

use crate::wing::{error::WingError, id::WingId, Wing, WingColor, WingDcaId};

#[derive(serde::Serialize, serde::Deserialize, specta::Type)]
pub struct WingChannelInfo {
    pub name: String,
    pub color: WingColor,
}

#[derive(
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
pub struct WingChannelId(u8);

impl std::fmt::Display for WingChannelId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl WingId for WingChannelId {
    type Id = u8;

    const MIN_ID: u8 = 1;
    const MAX_ID: u8 = 48;

    fn unchecked_new(id: u8) -> Self {
        Self(id)
    }

    fn value(&self) -> Self::Id {
        self.0
    }
}

impl TryFrom<u8> for WingChannelId {
    type Error = WingError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

pub struct WingChannel<'a> {
    wing: &'a Wing,
    id: WingChannelId,
}

impl<'a> WingChannel<'a> {
    pub fn new(wing: &'a Wing, id: WingChannelId) -> Self {
        Self { wing, id }
    }
}

// DCAs
impl<'a> WingChannel<'a> {
    fn get_channel_property(&self, property: &str) -> Option<i32> {
        let name = format!("/ch/{}/{}", self.id.display(), property);
        WingConsole::name_to_id(&name)
    }

    pub async fn get_tags(&self) -> Result<WingChannelTagList, WingError> {
        let tags = self
            .wing
            .request_string(self.get_channel_property("tags").unwrap())
            .await?;

        let tags = WingChannelTagList::new(tags.split(",").map(|tag| tag.to_string()));

        Ok(tags)
    }

    pub fn set_tags(&self, tags: WingChannelTagList) -> Result<(), WingError> {
        let data = tags.tags().into_iter().join(",");
        self.wing
            .set_string(self.get_channel_property("tags").unwrap(), &data)?;
        Ok(())
    }

    pub async fn assign_to_dca(&self, dca_id: WingDcaId) -> Result<(), WingError> {
        let mut tags = self.get_tags().await?;
        tags.add_dca(dca_id);
        self.set_tags(tags)?;
        Ok(())
    }

    pub async fn unassign_from_dca(&self, dca_id: WingDcaId) -> Result<(), WingError> {
        let mut tags = self.get_tags().await?;
        tags.remove_dca(dca_id);
        self.set_tags(tags)?;
        Ok(())
    }

    pub async fn set_dcas(
        &self,
        dcas: impl IntoIterator<Item = WingDcaId>,
    ) -> Result<(), WingError> {
        let mut tags = self.get_tags().await?;
        tags.set_dcas(dcas);
        self.set_tags(tags)?;
        Ok(())
    }

    pub fn mute(&self) -> Result<(), WingError> {
        todo!("wing channel mute")
    }

    pub fn unmute(&self) -> Result<(), WingError> {
        todo!("wing channel unmute")
    }

    pub fn set_pan(&self, pan: f32) -> Result<(), WingError> {
        todo!("wing channel set_pan")
    }

    pub async fn get_name(&self) -> Result<String, WingError> {
        let name = self
            .wing
            .request_string(self.get_channel_property("name").unwrap())
            .await?;
        Ok(name)
    }

    pub async fn get_color(&self) -> Result<WingColor, WingError> {
        let int_data = self
            .wing
            .request_int(self.get_channel_property("color").unwrap())
            .await? as u8;

        Ok(int_data.try_into().unwrap())
    }

    pub async fn get_info(&self) -> Result<WingChannelInfo, WingError> {
        let name = self.get_name().await?;
        let color = self.get_color().await?;
        Ok(WingChannelInfo { name, color })
    }
}

pub struct WingChannelTagList {
    tags: HashSet<String>,
}

impl WingChannelTagList {
    pub fn new(tags: impl IntoIterator<Item = String>) -> Self {
        Self {
            tags: tags.into_iter().collect(),
        }
    }

    fn dca_tag(dca_id: WingDcaId) -> String {
        format!("#D{}", dca_id.display())
    }

    pub fn add_dca(&mut self, dca_id: WingDcaId) {
        self.tags.insert(Self::dca_tag(dca_id));
    }

    pub fn remove_dca(&mut self, dca_id: WingDcaId) {
        self.tags.remove(&Self::dca_tag(dca_id));
    }

    pub fn clear_dcas(&mut self) {
        self.tags.retain(|tag| !tag.starts_with("#D"));
    }

    pub fn set_dcas(&mut self, dcas: impl IntoIterator<Item = WingDcaId>) {
        self.clear_dcas();

        for dca_id in dcas {
            self.add_dca(dca_id);
        }
    }

    pub fn tags(self) -> HashSet<String> {
        self.tags
    }
}

impl std::fmt::Display for WingChannelTagList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.tags.fmt(f)
    }
}
