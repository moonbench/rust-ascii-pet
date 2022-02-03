pub mod state;

use rand::Rng;

use std::fmt;

use crate::character::glyphs;

#[derive(Debug, Clone)]
pub enum Emotions {
    Angry,
    Anxious,
    Bored,
    Busy,
    Cheeky,
    Confused,
    Content,
    Creative,
    Curious,
    Distant,
    Empty,
    Excited,
    Frightened,
    Frustrated,
    Happy,
    Lonely,
    Loving,
    Playful,
    Sad,
    Stressed,
    Tired,
}

impl fmt::Display for Emotions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct Emotion {
    pub name: Emotions,
}

impl Emotion {
    pub fn pick_expression(&self) -> String {
        match self.name {
            Emotions::Angry => glyphs::angry_face(),
            Emotions::Anxious => glyphs::anxious_face(),
            Emotions::Bored => glyphs::bored_face(),
            Emotions::Cheeky => glyphs::random_cheeky_face(),
            Emotions::Content => glyphs::random_content_face(),
            Emotions::Curious => glyphs::random_curious_face(),
            Emotions::Happy => glyphs::random_happy_face(),
            Emotions::Excited => glyphs::random_excited_face(),
            Emotions::Frightened => glyphs::random_frightened_face(),
            Emotions::Frustrated => glyphs::frustrated_face(),
            Emotions::Loving => glyphs::random_loving_face(),
            Emotions::Sad => glyphs::random_sad_face(),
            Emotions::Playful => glyphs::playful_face(),
            Emotions::Tired => glyphs::tired_face(),
            _ => glyphs::null_face()
        }.to_string()
    }

    pub fn pick_ears(&self) -> String {
        match self.name {
            Emotions::Sad => glyphs::small_ears(),
            Emotions::Playful => glyphs::sport_ears(),
            _ => glyphs::default_ears()
        }.to_string()
    }

    pub fn name_string(&self) -> String {
        self.name.to_string()
    }

    pub fn next_emotion(&self) -> Emotions {

        if rand_range(0, 30) >= 28 {
            return Emotions::Frightened;
        }
        match self.name {
            Emotions::Angry => state::next_from_angry(),
            Emotions::Anxious => state::next_from_anxious(),
            Emotions::Bored => state::next_from_bored(),
            Emotions::Busy => state::next_from_busy(),
            Emotions::Cheeky => state::next_from_cheeky(),
            Emotions::Confused => state::next_from_confused(),
            Emotions::Content => state::next_from_content(),
            Emotions::Creative => state::next_from_creative(),
            Emotions::Curious => state::next_from_curious(),
            Emotions::Distant => state::next_from_distant(),
            Emotions::Empty => state::next_from_empty(),
            Emotions::Excited => state::next_from_excited(),
            Emotions::Frightened => state::next_from_frightened(),
            Emotions::Frustrated => state::next_from_frustrated(),
            Emotions::Happy => state::next_from_happy(),
            Emotions::Lonely => state::next_from_lonely(),
            Emotions::Loving => state::next_from_loving(),
            Emotions::Playful => state::next_from_playful(),
            Emotions::Sad => state::next_from_sad(),
            Emotions::Stressed => state::next_from_stressed(),
            Emotions::Tired => state::next_from_tired()
        }
    }
}

fn rand_range(min: u64, max: u64) -> u64 {
    rand::thread_rng().gen_range(min, max)
}
