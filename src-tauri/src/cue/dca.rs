use crate::{
    cue::CueExecutionContext,
    mix::{error::MixError, ActorId},
    wing::{WingColor, WingDcaId},
};

const DEFAULT_DCA_COLOR: WingColor = WingColor::Red;

#[derive(Clone, Default, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct DcaAssignment {
    assignment: [Option<ActorId>; 16],
}

impl DcaAssignment {
    pub async fn apply<'a>(&self, context: &CueExecutionContext<'a>) -> Result<(), MixError> {
        let assignments = self
            .assignment
            .map(|a| a.and_then(|actor_id| context.config.actor(actor_id).ok()));

        // Mute and unassign unused channels
        // TODO: find a cleaner way to do this
        for channel_id in context.config.controlled_channels().filter(|channel_id| {
            !assignments
                .iter()
                .any(|actor| actor.is_some_and(|actor| *actor.channel() == **channel_id))
        }) {
            let channel = context.wing.channel(*channel_id);
            channel.mute()?;
            channel.set_dcas([]).await?;
        }

        // Unmute and assign current channels
        for (dca_idx, actor) in assignments.into_iter().enumerate() {
            let Some(actor) = actor else { continue };

            let dca_id: WingDcaId = ((dca_idx + 1) as u8).try_into().unwrap();

            let dca = context.wing.dca(dca_id);
            dca.set_name(actor.name())?;
            dca.set_color(actor.color().unwrap_or(DEFAULT_DCA_COLOR))?;

            let channel = context.wing.channel(*actor.channel());

            channel.unmute()?;
            channel.set_dcas([dca_id]).await?;
        }

        Ok(())
    }
}
