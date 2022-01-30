mod emotion;
mod animation;

use crate::character::emotion::{Emotion, Emotions};
use crate::character::animation::{Animation};

#[derive(Debug)]
pub struct Character<'a> {
    pub name: String,
    pub face: String,
    pub emotion: Emotion<'a>,
    pub animation: Animation
}

impl Character<'_>  {
    pub fn default() -> Character<'static> {
        let emotion = Emotion {
            name: &Emotions::Null
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
        self.emotion = Emotion{
            name: &Emotions::Happy
        };
        self.face = self.emotion.pick_expression();
        self.animation = Animation::make_for(&self.emotion, &self.emotion.pick_expression());
    }
}

fn empty_layer() {
    print!("║{: ^30}║\n", "");
}
