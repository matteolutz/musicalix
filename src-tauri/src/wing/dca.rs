use libwing::WingConsole;

use crate::wing::{error::WingError, id::WingId, Wing, WingColor};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct WingDcaId(u8);

impl WingId for WingDcaId {
    type Id = u8;

    const MIN_ID: u8 = 1;
    const MAX_ID: u8 = 16;

    fn unchecked_new(id: u8) -> Self {
        Self(id)
    }

    fn value(&self) -> u8 {
        self.0
    }
}

impl TryFrom<u8> for WingDcaId {
    type Error = WingError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

pub struct WingDca<'a> {
    wing: &'a Wing,
    id: WingDcaId,
}

impl<'a> WingDca<'a> {
    pub fn new(wing: &'a Wing, id: WingDcaId) -> Self {
        Self { wing, id }
    }
}

// DCAs
impl<'a> WingDca<'a> {
    fn get_dca_property(&self, property: &str) -> Option<i32> {
        let name = format!("/dca/{}/{}", self.id.display(), property);
        WingConsole::name_to_id(&name)
    }

    pub fn set_name(&self, new_name: &str) -> Result<(), WingError> {
        self.wing
            .set_string(self.get_dca_property("name").unwrap(), new_name)?;
        Ok(())
    }

    pub fn set_color(&self, color: WingColor) -> Result<(), WingError> {
        self.wing
            .set_int(self.get_dca_property("col").unwrap(), color as i32)?;
        Ok(())
    }
}
