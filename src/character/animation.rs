use crate::character::emotion::{Emotion, Emotions};

use rand::Rng;

#[derive(Debug, Clone)]
pub struct Frame(pub String, pub u64);

impl Frame {
    fn new(text: String, duration: u64) -> Frame {
        Frame(text, duration)
    }
}

#[derive(Debug)]
pub struct Animation {
    pub frames: Vec<Frame>,
    pub current: usize,
    pub duration: u64,
    pub loops: u64,
}

impl Animation {
    fn new(frames: &[Frame], loops: u64) -> Animation {
        Animation {
            frames: frames.to_vec(),
            current: 0,
            duration: frames.iter().fold(0, |accum, frame| accum + frame.1 ) * loops,
            loops
        }
    }
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
            Emotions::Happy => random_happy(face),
            Emotions::Excited => excite(face),
            Emotions::Playful => play(face),
            Emotions::Loving => give_love(face),
            _ => nada(face)
        }
    }
}

fn rand_range(min: u64, max: u64) -> u64 {
    rand::thread_rng().gen_range(min, max)
}

fn random_happy(face: &str) -> Animation {
    match rand_range(0,100) {
        0..=30 => look_around(face),
        31..=40 => sit(face),
        51..=80 => pulse(face),
        81..=90 => excite(face),
        _ => nada(face),
    }
}

fn sit(face: &str) -> Animation {
    Animation::new(&[
        Frame::new(format!("( {} )", face), rand_range(2000,5000)),
    ], 1)
}

fn look_around(face: &str) -> Animation {
    Animation::new(&[
        Frame::new(format!("( {} )", face), rand_range(700,2000)),
        Frame::new(format!("(  {})", face), rand_range(250,1000)),
        Frame::new(format!("( {} )", face), rand_range(250,1000)),
        Frame::new(format!("({}  )", face), rand_range(250,1000)),
        Frame::new(format!("( {} )", face), rand_range(700,2000)),
        Frame::new(format!("(  {})", face), rand_range(250,1000)),
        Frame::new(format!("( {} )", face), rand_range(250,1000)),
        Frame::new(format!("({}  )", face), rand_range(450,1000)),
    ], rand_range(2,4))
}

fn pulse(face: &str) -> Animation {
    Animation::new(&[
        Frame::new(format!("( {} )", face), 1250),
        Frame::new(format!("(  {}  )", face), 1250),
    ], rand_range(2,6))
}

fn excite(face: &str) -> Animation {
    Animation::new(&[
        Frame::new(format!(" _( {} )_ ", face), 500),
        Frame::new(format!("~\\( {} )/~", face), 250),
        Frame::new(format!("☆ ☆\\( {} )/☆ ☆", face), 500),
    ], rand_range(2,4))
}

fn nada(face: &str) -> Animation {
    Animation::new(&[
        Frame::new(format!("   ( {} )   ", face), 500),
        Frame::new(format!("   ( {} )?  ", face), 500),
        Frame::new(format!("   ( {} ) ? ", face), 500),
        Frame::new(format!("   ( {} )  ?", face), 500),
    ], rand_range(2,5))
}

fn play(_face: &str) -> Animation {
    Animation::new(&[
        Frame::new(format!("     ( •_• )     "), 2000),
        Frame::new(format!("     (  •_•)     "), 2000),
        Frame::new(format!("     (  •_•)>⌐■-■"), 450),
        Frame::new(format!("     (  •_•)⌐■-■ "), 300),
        Frame::new(format!("     (  •⌐■-■    "), 250),
        Frame::new(format!("     ( ⌐■_■)     "), 5000),
    ], 1)
}

fn give_love(face: &str) -> Animation {
    Animation::new(&[
        Frame::new(format!(" ( {} ) ", face), 500),
        Frame::new(format!(" (  {}) ", face), 500),
        Frame::new(format!(" (  {})♥", face), 500),
        Frame::new(format!(" ( {} )♥", face), 1000),
    ], rand_range(2,3))
}
