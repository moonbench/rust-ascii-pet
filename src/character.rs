pub mod emotion;
pub mod animation;
pub mod glyphs;

use crate::character::emotion::{Emotion, Emotions};
use crate::character::animation::{Animation};

#[derive(Debug)]
pub struct Character {
    pub name: String,
    pub face: String,
    pub ears: String,
    pub emotion: Emotion,
    pub animation: Animation
}

impl Character  {
    pub fn default() -> Character {
        let emotion = Emotion {
            name: Emotions::Content
        };
        Character {
            name: "Buddy".to_string(),
            face: emotion.pick_expression(),
            ears: "ᑎ___ᑎ".to_string(),
            animation: Animation::make_for(&emotion, &emotion.pick_expression()),
            emotion: emotion,
        }
    }

    pub fn next_tick(&mut self) -> u64 {
        for _ in 0..6 {
            empty_layer();
        }
        print!("║{:^30}║\n", &self.ears);
        print!("║{:^30}║\n", &self.animation.frame().0);
        let delay = self.animation.frame().1;
        self.animation.next();
        delay
    }

    pub fn set_emotion(&mut self, name: Emotions){
        self.emotion = Emotion{name: name.clone()};
        self.face = self.emotion.pick_expression();
        self.ears = self.emotion.pick_ears();
        self.animation = Animation::make_for(&self.emotion, &self.face);
    }

    pub fn state_change(&mut self) {
        self.set_emotion(self.emotion.next_emotion());
    }
}

fn empty_layer() {
    print!("║{: ^30}║\n", "");
}
