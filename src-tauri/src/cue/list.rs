use itertools::Itertools;

use crate::cue::{Cue, CueId};

#[derive(Clone, Default, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(transparent)]
pub struct CueList(Vec<Cue>);

impl CueList {
    fn sort(&mut self) {
        self.0.sort_by_key(|cue| cue.id);
    }

    pub fn push(&mut self, cue: Cue) -> usize {
        for i in 0..self.0.len() {
            if self.0[i].id > cue.id {
                self.0.insert(i, cue);
                return i;
            }
        }

        self.0.push(cue);
        self.0.len() - 1
    }

    pub fn iter(&self) -> impl Iterator<Item = &Cue> {
        self.0.iter()
    }

    pub fn has(&self, cue_id: &CueId) -> bool {
        self.0.iter().any(|c| &c.id == cue_id)
    }

    pub fn get(&self, cue_id: &CueId) -> Option<&Cue> {
        self.0.iter().find(|cue| &cue.id == cue_id)
    }

    pub fn get_mut(&mut self, cue_id: &CueId) -> Option<&mut Cue> {
        self.0.iter_mut().find(|cue| &cue.id == cue_id)
    }

    pub fn remove(&mut self, cue_id: &CueId) -> Option<Cue> {
        let (idx, _) = self.0.iter().find_position(|c| &c.id == cue_id)?;
        Some(self.0.remove(idx))
    }
}
