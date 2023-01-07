pub enum Scale {
    Major,
    Minor,
    Pentatonic,
    Blues,
    Chromatic,
    WholeTone,
    Octatonic,
    Diminished,
    HarmonicMinor,
    MelodicMinor,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
}

pub struct ScaleGenerator {}

impl ScaleGenerator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate(&self, scale: Scale) -> Vec<u8> {
        match scale {
            Scale::Major => vec![0, 2, 4, 5, 7, 9, 11],
            Scale::Minor => vec![0, 2, 3, 5, 7, 8, 10],
            Scale::Pentatonic => vec![0, 2, 4, 7, 9],
            Scale::Blues => vec![0, 3, 5, 6, 7, 10],
            Scale::Chromatic => vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            Scale::WholeTone => vec![0, 2, 4, 6, 8, 10],
            Scale::Octatonic => vec![0, 2, 3, 5, 6, 8, 9, 11],
            Scale::Diminished => vec![0, 2, 3, 5, 6, 8, 9, 11],
            Scale::HarmonicMinor => vec![0, 2, 3, 5, 7, 8, 11],
            Scale::MelodicMinor => vec![0, 2, 3, 5, 7, 9, 11],
            Scale::Dorian => vec![0, 2, 3, 5, 7, 9, 10],
            Scale::Phrygian => vec![0, 1, 3, 5, 7, 8, 10],
            Scale::Lydian => vec![0, 2, 4, 6, 7, 9, 11],
            Scale::Mixolydian => vec![0, 2, 4, 5, 7, 9, 10],
        }
    }
}
