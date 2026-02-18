use std::collections::HashMap;

use crate::{
    cue::CueExecutionContext,
    mix::{error::MixError, ActorId, PositionId},
};

#[derive(Clone, Default, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct PositionAssignment {
    assignment: HashMap<ActorId, PositionId>,
}

impl PositionAssignment {
    pub fn apply(&self, context: &CueExecutionContext) -> Result<(), MixError> {
        for (actor_id, actor) in context.config.actors() {
            let position_id = self.assignment.get(actor_id);
            let position = position_id.and_then(|id| context.config.position(*id).ok());
            let position = position.cloned().unwrap_or_default();

            let channel = context.wing.channel(*actor.channel());
            position.apply(&channel)?;
        }

        Ok(())
    }
}
