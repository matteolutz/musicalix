use libwing::WingConsole;

use crate::wing::{error::WingError, WingColor};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct WingDcaId(u8);

impl WingDcaId {
    const MIN_DCA: u8 = 1;
    const MAX_DCA: u8 = 16;
}

impl TryFrom<u8> for WingDcaId {
    type Error = WingError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value >= Self::MIN_DCA && value <= Self::MAX_DCA {
            Ok(WingDcaId(value))
        } else {
            Err(WingError::id_out_of_bounds(
                value,
                Self::MIN_DCA,
                Self::MAX_DCA,
            ))
        }
    }
}

impl WingDcaId {
    pub fn to_u8(&self) -> u8 {
        self.0
    }
}

pub struct WingDca<'a> {
    wing: &'a mut WingConsole,
    id: WingDcaId,
}

impl<'a> WingDca<'a> {
    pub fn new(wing: &'a mut WingConsole, id: WingDcaId) -> Self {
        Self { wing, id }
    }
}

// DCAs
impl<'a> WingDca<'a> {
    fn get_dca_property(&self, property: &str) -> Option<i32> {
        let name = format!("/dca/{}/{}", self.id.to_u8(), property);
        WingConsole::name_to_id(&name)
    }

    pub fn set_name(&mut self, new_name: &str) -> Result<(), WingError> {
        self.wing
            .set_string(self.get_dca_property("name").unwrap(), new_name)?;
        Ok(())
    }

    pub fn set_color(&mut self, color: WingColor) -> Result<(), WingError> {
        self.wing
            .set_int(self.get_dca_property("col").unwrap(), color as i32)?;
        Ok(())
    }
}
