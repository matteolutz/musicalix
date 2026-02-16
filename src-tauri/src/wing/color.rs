#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, specta::Type)]
pub enum WingColor {
    GrayBlue = 1,
    MediumBlue = 2,
    DarkBlue = 3,
    Turquoise = 4,
    Green = 5,
    OliveGreen = 6,
    Yellow = 7,
    Orange = 8,
    Red = 9,
    Coral = 10,
    Pink = 11,
    Mauve = 12,
}

impl TryFrom<u8> for WingColor {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(WingColor::GrayBlue),
            2 => Ok(WingColor::MediumBlue),
            3 => Ok(WingColor::DarkBlue),
            4 => Ok(WingColor::Turquoise),
            5 => Ok(WingColor::Green),
            6 => Ok(WingColor::OliveGreen),
            7 => Ok(WingColor::Yellow),
            8 => Ok(WingColor::Orange),
            9 => Ok(WingColor::Red),
            10 => Ok(WingColor::Coral),
            11 => Ok(WingColor::Pink),
            12 => Ok(WingColor::Mauve),
            _ => Err(()),
        }
    }
}
