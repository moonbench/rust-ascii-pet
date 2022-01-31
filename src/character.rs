pub mod emotion;
mod animation;

use crate::character::emotion::{Emotion, Emotions};
use crate::character::animation::{Animation};

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
            name: "Powa".to_string(),
            face: emotion.pick_expression(),
            animation: Animation::make_for(&emotion, &emotion.pick_expression()),
            emotion: emotion,
        }
    }

    pub fn next_tick(&mut self) -> u64 {
        empty_layer();
        empty_layer();
        empty_layer();
        empty_layer();
        print!("║{:^30}║\n", &self.animation.frame().0);
        let delay = self.animation.frame().1;
        self.animation.next();
        delay
    }

    pub fn pick_emotion(&mut self){
        self.set_emotion(Emotions::Happy);
    }

    pub fn set_emotion(&mut self, name: Emotions){
        self.emotion = Emotion{name: name.clone()};
        self.face = self.emotion.pick_expression();
        self.animation = Animation::make_for(&self.emotion, &self.emotion.pick_expression());
    }
}

fn empty_layer() {
    print!("║{: ^30}║\n", "");
}
