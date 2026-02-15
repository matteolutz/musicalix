use std::{collections::HashSet, fmt::Debug};

use itertools::Itertools;
use libwing::WingConsole;

use crate::wing::{error::WingError, WingConsoleExt, WingDcaId};

pub struct WingInputChannel<'a> {
    wing: &'a mut WingConsole,
    id: u32,
}

impl<'a> WingInputChannel<'a> {
    pub fn new(wing: &'a mut WingConsole, id: u32) -> Self {
        Self { wing, id }
    }
}

// DCAs
impl<'a> WingInputChannel<'a> {
    fn get_channel_property(&self, property: &str) -> Option<i32> {
        let name = format!("/ch/{}/{}", self.id, property);
        WingConsole::name_to_id(&name)
    }

    pub fn get_tags(&mut self) -> Result<WingInputChannelTagList, WingError> {
        let data = self
            .wing
            .request_and_read_data(self.get_channel_property("tags").unwrap())?;

        let tags =
            WingInputChannelTagList::new(data.get_string().split(",").map(|tag| tag.to_string()));

        Ok(tags)
    }

    pub fn set_tags(&mut self, tags: WingInputChannelTagList) -> Result<(), WingError> {
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
}

pub struct WingInputChannelTagList {
    tags: HashSet<String>,
}

impl WingInputChannelTagList {
    pub fn new(tags: impl IntoIterator<Item = String>) -> Self {
        Self {
            tags: tags.into_iter().collect(),
        }
    }

    pub fn add_dca(&mut self, dca_id: WingDcaId) {
        self.tags.insert(format!("#D{}", dca_id.to_u8()));
    }

    pub fn remove_dca(&mut self, dca_id: WingDcaId) {
        self.tags.remove(&format!("#D{}", dca_id.to_u8()));
    }

    pub fn tags(self) -> HashSet<String> {
        self.tags
    }
}

impl std::fmt::Display for WingInputChannelTagList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.tags.fmt(f)
    }
}
