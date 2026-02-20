use std::collections::HashSet;

use crate::{
    cue::CueExecutionContext,
    mix::{error::MixError, ActorId, GroupId},
    wing::{WingChannelId, WingColor, WingDcaId},
};

const DEFAULT_DCA_COLOR: WingColor = WingColor::Red;

#[derive(Default)]
struct SingleDcaAssignmentBake {
    channels: HashSet<WingChannelId>,
    color: Option<WingColor>,
    name: String,
}

#[derive(Copy, Clone, Default, serde::Serialize, serde::Deserialize, specta::Type)]
pub enum SingleDcaAssignment {
    #[default]
    None,

    Actor(ActorId),
    Group(GroupId),
}

impl SingleDcaAssignment {
    fn bake_assignment(&self, context: &CueExecutionContext) -> SingleDcaAssignmentBake {
        match self {
            Self::None => SingleDcaAssignmentBake::default(),
            Self::Actor(actor_id) => context
                .config
                .actor(*actor_id)
                .map(|actor| SingleDcaAssignmentBake {
                    channels: [*actor.channel()].into(),
                    color: actor.color(),
                    name: actor.name().to_string(),
                })
                .unwrap_or_default(),
            Self::Group(group_id) => context
                .config
                .group(*group_id)
                .map(|group| {
                    let channels = group
                        .actors()
                        .filter_map(|actor_id| context.config.actor(*actor_id).ok())
                        .map(|actor| *actor.channel())
                        .collect();

                    SingleDcaAssignmentBake {
                        channels,
                        color: group.color(),
                        name: group.name().to_string(),
                    }
                })
                .unwrap_or_default(),
        }
    }
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct DcaAssignment {
    pub(super) assignment: [SingleDcaAssignment; 16],
}

impl DcaAssignment {
    pub async fn apply<'a>(&self, context: &CueExecutionContext<'a>) -> Result<(), MixError> {
        let assignments = self.assignment.map(|a| a.bake_assignment(context));

        // Mute and unassign unused channels
        // TODO: find a cleaner way to do this
        for channel_id in context
            .config
            .controlled_channels()
            .filter(|channel_id| !assignments.iter().any(|a| a.channels.contains(channel_id)))
        {
            let channel = context.wing.channel(*channel_id);
            channel.mute()?;
            channel.set_dcas([]).await?;
        }

        // Unmute and assign current channels
        for (dca_idx, assignment) in assignments.into_iter().enumerate() {
            let dca_id: WingDcaId = ((dca_idx + 1) as u8).try_into().unwrap();

            let dca = context.wing.dca(dca_id);
            dca.set_name(&assignment.name)?;
            dca.set_color(assignment.color.unwrap_or(DEFAULT_DCA_COLOR))?;

            for channel in assignment.channels.into_iter() {
                let channel = context.wing.channel(channel);

                channel.unmute()?;
                channel.set_dcas([dca_id]).await?;
            }
        }

        Ok(())
    }
}
