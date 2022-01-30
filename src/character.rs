mod emotion;
use crate::character::emotion::{Emotion, Emotions};

use rand::thread_rng;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Frame(pub String, pub u64);

#[derive(Debug)]
pub struct Animation {
    pub frames: Vec<Frame>,
    pub current: usize,
}

impl Animation {
    pub fn frame(&self) -> &Frame {
        let frame = &self.frames[self.current];
        frame
    }
    pub fn next(&mut self){
        self.current += 1;
        if self.current >= self.frames.len() {
            self.current = 0;
        }
    }
    pub fn make_for(emotion: &Emotion, face: &str) -> Animation{
        Animation {
            frames: [
                Frame(format!("( {} )", face).to_string(), 1000),
                Frame(format!("(  {})", face).to_string(), 500),
                Frame(format!("( {} )", face).to_string(), 1000),
                Frame(format!("({}  )", face).to_string(), 500),
            ].to_vec(),
            current: 0,
        }
    }
}

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
        self.print(&self.animation.frame().0);
        let delay = self.animation.frame().1;
        self.animation.next();
        delay
    }

    fn print(&self, text: &String){
        print!("{:^24}", text);
    }

    pub fn pick_emotion(&mut self){
        self.emotion = Emotion{
            name: &Emotions::Happy
        };
        self.face = self.emotion.pick_expression();
        self.animation = Animation::make_for(&self.emotion, &self.emotion.pick_expression());
    }
}
