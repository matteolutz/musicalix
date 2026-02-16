use libwing::WingConsole;

use crate::{
    mix::{error::MixError, ActorId, MixConfig},
    wing::{WingColor, WingConsoleExt, WingDcaId},
};

const DEFAULT_DCA_COLOR: WingColor = WingColor::Red;

#[derive(Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct DcaAssignment {
    assignment: [Option<ActorId>; 16],
}

impl DcaAssignment {
    pub fn apply(&self, config: &MixConfig, wing: &mut WingConsole) -> Result<(), MixError> {
        let assignments = self
            .assignment
            .map(|a| a.and_then(|actor_id| config.actor(actor_id).ok()));

        // Mute and unassign unused channels
        // TODO: find a cleaner way to do this
        for channel_id in config.controlled_channels().filter(|channel_id| {
            !assignments
                .iter()
                .any(|actor| actor.is_some_and(|actor| *actor.channel() == **channel_id))
        }) {
            let mut channel = wing.channel(*channel_id);
            channel.mute()?;
            channel.set_dcas([])?;
        }

        // Unmute and assign current channels
        for (dca_idx, actor) in assignments.into_iter().enumerate() {
            let Some(actor) = actor else { continue };

            let dca_id: WingDcaId = ((dca_idx + 1) as u8).try_into().unwrap();

            let mut dca = wing.dca(dca_id);
            dca.set_name(actor.name())?;
            dca.set_color(actor.color().unwrap_or(DEFAULT_DCA_COLOR))?;

            let mut channel = wing.channel(*actor.channel());

            channel.unmute()?;
            channel.set_dcas([dca_id])?;
        }

        Ok(())
    }
}
