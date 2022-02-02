use rand::seq::SliceRandom;

use crate::character::glyphs;

#[derive(Debug, Clone)]
pub enum Emotions {
    Happy,
    Excited,
    Loving,
    Sad,
    Playful,
    Null
}

#[derive(Debug)]
pub struct Emotion {
    pub name: Emotions,
}

impl Emotion {
    pub fn pick_expression(&self) -> String {
        match self.name {
            Emotions::Happy => glyphs::random_happy_face(),
            Emotions::Excited => glyphs::random_excited_face(),
            Emotions::Loving => glyphs::random_loving_face(),
            Emotions::Sad => glyphs::random_sad_face(),
            Emotions::Null => glyphs::null_face(),
            Emotions::Playful => glyphs::playful_face(),
        }.to_string()
    }

    pub fn pick_ears(&self) -> String {
        match self.name {
            Emotions::Sad | Emotions::Null => glyphs::small_ears(),
            Emotions::Playful => glyphs::sport_ears(),
            _ => glyphs::default_ears()
        }.to_string()
    }

    pub fn name_string(&self) -> String {
        match self.name {
            Emotions::Happy => "happy",
            Emotions::Excited => "excited",
            Emotions::Loving => "loving",
            Emotions::Sad => "sad",
            Emotions::Null => "nothing",
            Emotions::Playful => "playful"
        }.to_string()
    }
}
