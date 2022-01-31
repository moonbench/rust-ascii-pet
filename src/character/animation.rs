use crate::character::emotion::{Emotion, Emotions};

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
        match emotion.name {
            Emotions::Happy => idle(face),
            _ => nada(face)
        }
    }
}

fn idle(face: &str) -> Animation {
    Animation {
        frames: [
            Frame(format!("( {} )", face).to_string(), 500),
            Frame(format!("(  {})", face).to_string(), 500),
            Frame(format!("( {} )", face).to_string(), 500),
            Frame(format!("({}  )", face).to_string(), 500),
        ].to_vec(),
        current: 0,
    }
}


fn nada(face: &str) -> Animation {
    Animation {
        frames: [
            Frame(format!("[ {} ]", face).to_string(), 1000),
            Frame(format!("[[ {} ]]", face).to_string(), 1000),
        ].to_vec(),
        current: 0,
    }
}
