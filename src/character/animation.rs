use crate::character::emotion::{Emotion};

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
    pub fn make_for(_emotion: &Emotion, face: &str) -> Animation{
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
