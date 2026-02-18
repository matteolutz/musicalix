use crate::cue::Cue;

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
}
