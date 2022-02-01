pub mod emotion;
mod animation;

use crate::character::emotion::{Emotion, Emotions};
use crate::character::animation::{Animation};

use rand::Rng;

#[derive(Debug)]
pub struct Character {
    pub name: String,
    pub face: String,
    pub emotion: Emotion,
    pub animation: Animation
}

impl Character  {
    pub fn default() -> Character {
        let emotion = Emotion {
            name: Emotions::Null
        };
        Character {
            name: "Buddy".to_string(),
            face: emotion.pick_expression(),
            animation: Animation::make_for(&emotion, &emotion.pick_expression()),
            emotion: emotion,
        }
    }

    pub fn next_tick(&mut self) -> u64 {
        for _ in 0..6 {
            empty_layer();
        }
        print!("║{:^30}║\n", "ᑎ___ᑎ");
        print!("║{:^30}║\n", &self.animation.frame().0);
        let delay = self.animation.frame().1;
        self.animation.next();
        delay
    }

    pub fn set_emotion(&mut self, name: Emotions){
        self.emotion = Emotion{name: name.clone()};
        self.face = self.emotion.pick_expression();
        self.animation = Animation::make_for(&self.emotion, &self.face);
    }

    pub fn state_change(&mut self) {
        match rand::thread_rng().gen_range(0,100) {
            0..= 50 => self.set_emotion(Emotions::Happy),
            51..=60 => self.set_emotion(Emotions::Sad),
            61..=70 => self.set_emotion(Emotions::Excited),
            71..=80 => self.set_emotion(Emotions::Playful),
            81..=95 => self.set_emotion(Emotions::Loving),
            _ => self.set_emotion(Emotions::Null),
        }

    }
}

fn empty_layer() {
    print!("║{: ^30}║\n", "");
}
