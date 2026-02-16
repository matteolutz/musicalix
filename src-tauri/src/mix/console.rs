use libwing::WingConsole;

use crate::mix::{error::MixError, DcaAssignment, MixConfig};

pub trait MixWingConsoleExt {
    fn apply_dca_assignment(
        &mut self,
        assignment: &DcaAssignment,
        config: &MixConfig,
    ) -> Result<(), MixError>;
}

impl MixWingConsoleExt for WingConsole {
    fn apply_dca_assignment(
        &mut self,
        assignment: &DcaAssignment,
        config: &MixConfig,
    ) -> Result<(), MixError> {
        assignment.apply(config, self)
    }
}
