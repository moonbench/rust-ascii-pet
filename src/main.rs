use std::thread;
use std::time::Duration;
use std::io::Write;

#[derive(Debug)]
struct Emotion {
    name: String,
}

impl Emotion {
    fn get_expression(&self) -> String {
        "^‿^".to_string()
    }
    fn get_animation(&self, face: String) -> Animation {
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
struct Frame(String, u64);

#[derive(Debug)]
struct Animation {
    frames: Vec<Frame>,
    current: usize,
}

impl Animation {
    fn frame(&self) -> &Frame {
        let frame = &self.frames[self.current];
        frame
    }
    fn next(&mut self){
        self.current += 1;
        if self.current >= self.frames.len() {
            self.current = 0;
        }
    }
}

#[derive(Debug)]
struct Character {
    name: String,
    face: String,
    emotion: Emotion,
    animation: Animation
}

impl Character {
    fn next_tick(&mut self) -> u64 {
        self.print(&self.animation.frame().0);
        let delay = self.animation.frame().1;
        self.animation.next();
        delay
    }


    fn print(&self, text: &String){
        print!("{:^24}", text);
    }

    fn pick_emotion(&mut self){
        self.emotion = Emotion{
            name: "Happy".to_string()
        };
        self.face = self.emotion.get_expression();
    }
}

fn main() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    let emotion = Emotion {
        name: "Null".to_string()
    };

    let mut pet = Character {
        name: "Powa".to_string(),
        face: emotion.get_expression(),
        animation: emotion.get_animation(emotion.get_expression()),
        emotion: emotion,
    };

    pet.pick_emotion();

    loop {
        print!("{esc}c", esc = 27 as char);
        print!("╔{:═^22}╗\n\n\n", format!(" {} ", pet.name));
        let delay = pet.next_tick();
        print!("\n\n╩╦{:═^20}╦╩\n", "");
        std::io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(delay));
    }
    //
    // let faces = [
    //     "•_•",
    //     "•-•",
    //     "°^°",
    //     "⩾﹏⩽",
    //     "=^_^=",
    //     "・3・",
    //     "꒡⌓꒡",
    //     "=____=",
    //     "︶︹︶",
    //     "^‿^",
    //     "`Д´"
    // ];
    //
    // for face in faces {
    //     pet.expression = face.to_string();
    //     pet.look_around();
    //     pet.point_left();
    //     thread::sleep(Duration::from_millis(350));
    //     pet.point_right();
    //     thread::sleep(Duration::from_millis(350));
    //     pet.double_point_left();
    //     thread::sleep(Duration::from_millis(350));
    //     pet.double_point_right();
    //     thread::sleep(Duration::from_millis(350));
    // }
    //
}
