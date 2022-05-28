use std::ops::Range;


pub enum Quality {
    Broken,
    Damaged,
    Normal,
    Masterwork,
    Artifact,
}
impl Quality {
    /// broken (20% .. 60%),
    /// damaged (60% .. 90%),
    /// normal (90 .. 110%),
    /// masterwork (110% .. 140%),
    /// artifact (140% .. 200%)
    pub fn get_multiplier(&self) -> Range<u8> {
        match self {
            Quality::Broken => 20..60,
            Quality::Damaged => 60..90,
            Quality::Normal => 90..110,
            Quality::Masterwork => 110..140,
            Quality::Artifact => 140..200,
        }
    }
}
impl Default for Quality {
    fn default() -> Self {
        Self::Normal
    }
}