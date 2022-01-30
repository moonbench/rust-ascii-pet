
#[derive(Debug)]
pub struct Emotion {
    pub name: String,
}

impl Emotion {
    pub fn get_expression(&self) -> String {
        "^‿^".to_string()
    }
    pub fn get_animation(&self, face: String) -> Animation {
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
}

#[derive(Debug)]
pub struct Character {
    pub name: String,
    pub face: String,
    pub emotion: Emotion,
    pub animation: Animation
}

impl Character {
    pub fn default() -> Character {
        let emotion = Emotion {
            name: "Null".to_string()
        };
        Character {
            name: "Powa".to_string(),
            face: emotion.get_expression(),
            animation: emotion.get_animation(emotion.get_expression()),
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
            name: "Happy".to_string()
        };
        self.face = self.emotion.get_expression();
    }
}
