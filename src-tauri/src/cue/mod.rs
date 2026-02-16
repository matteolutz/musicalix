use crate::mix::DcaAssignment;

#[derive(Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub enum Cue {
    DcaAssignment(DcaAssignment),
}
