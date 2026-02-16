use std::{collections::HashSet, fmt::Debug};

use itertools::Itertools;
use libwing::WingConsole;

use crate::wing::{error::WingError, id::WingId, WingColor, WingConsoleExt, WingDcaId};

#[derive(serde::Serialize, serde::Deserialize, specta::Type)]
pub struct WingChannelInfo {
    pub name: String,
    pub color: WingColor,
}

#[derive(
    Copy, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize, specta::Type,
)]
pub struct WingChannelId(u8);

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
    wing: &'a mut WingConsole,
    id: WingChannelId,
}

impl<'a> WingChannel<'a> {
    pub fn new(wing: &'a mut WingConsole, id: WingChannelId) -> Self {
        Self { wing, id }
    }
}

// DCAs
impl<'a> WingChannel<'a> {
    fn get_channel_property(&self, property: &str) -> Option<i32> {
        let name = format!("/ch/{}/{}", self.id.display(), property);
        WingConsole::name_to_id(&name)
    }

    pub fn get_tags(&mut self) -> Result<WingChannelTagList, WingError> {
        let data = self
            .wing
            .request_and_read_data(self.get_channel_property("tags").unwrap())?;

        let tags = WingChannelTagList::new(data.get_string().split(",").map(|tag| tag.to_string()));

        Ok(tags)
    }

    pub fn set_tags(&mut self, tags: WingChannelTagList) -> Result<(), WingError> {
        let data = tags.tags().into_iter().join(",");
        self.wing
            .set_string(self.get_channel_property("tags").unwrap(), &data)?;
        Ok(())
    }

    pub fn assign_to_dca(&mut self, dca_id: WingDcaId) -> Result<(), WingError> {
        let mut tags = self.get_tags()?;
        tags.add_dca(dca_id);
        self.set_tags(tags)?;
        Ok(())
    }

    pub fn unassign_from_dca(&mut self, dca_id: WingDcaId) -> Result<(), WingError> {
        let mut tags = self.get_tags()?;
        tags.remove_dca(dca_id);
        self.set_tags(tags)?;
        Ok(())
    }

    pub fn set_dcas(&mut self, dcas: impl IntoIterator<Item = WingDcaId>) -> Result<(), WingError> {
        let mut tags = self.get_tags()?;
        tags.set_dcas(dcas);
        self.set_tags(tags)?;
        Ok(())
    }

    pub fn mute(&mut self) -> Result<(), WingError> {
        todo!()
    }

    pub fn unmute(&mut self) -> Result<(), WingError> {
        todo!()
    }

    pub fn get_name(&mut self) -> Result<String, WingError> {
        let data = self
            .wing
            .request_and_read_data(self.get_channel_property("name").unwrap())?;
        Ok(data.get_string())
    }

    pub fn get_color(&mut self) -> Result<WingColor, WingError> {
        let data = self
            .wing
            .request_and_read_data(self.get_channel_property("color").unwrap())?;
        let int_data = data.get_int() as u8;
        Ok(int_data.try_into().unwrap())
    }

    pub fn get_info(&mut self) -> Result<WingChannelInfo, WingError> {
        let name = self.get_name()?;
        let color = self.get_color()?;
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
