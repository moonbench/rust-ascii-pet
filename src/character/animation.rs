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
            Emotions::Angry => sit(face),
            Emotions::Anxious => sweat(face),
            Emotions::Bored => random_bored(face),
            Emotions::Busy => random_busy(face),
            Emotions::Cheeky => random_cheeky(face),
            Emotions::Confused => random_confused(face),
            Emotions::Content => random_content(face),
            Emotions::Creative => random_creative(face),
            Emotions::Curious => random_curious(face),
            Emotions::Distant => sit(face),
            Emotions::Empty => loaf(face),
            Emotions::Feeding => eat(face),
            Emotions::Excited => excite(face),
            Emotions::Frightened => sweat(face),
            Emotions::Frustrated => endure(face),
            Emotions::Happy => random_happy(face),
            Emotions::Lonely => cry(face),
            Emotions::Loving => give_love(face),
            Emotions::Playful => random_play(face),
            Emotions::Sad => random_sad(face),
            Emotions::Stressed => sweat(face),
            Emotions::Tired => random_tired(face),
        }
    }
}

fn rand_range(min: u64, max: u64) -> u64 {
    rand::thread_rng().gen_range(min, max)
}

fn random_bored(face: &str) -> Animation {
    match rand_range(0,100) {
        0..=20 => sit(face),
        21..=40 => tired(face),
        _ => loaf(face),
    }
}

fn random_busy(face: &str) -> Animation {
    match rand_range(0,100) {
        0..=40 => make(face),
        41..=50 => observe(face),
        _ => sweat(face),
    }
}

fn random_happy(face: &str) -> Animation {
    match rand_range(0,100) {
        0..=14 => look_around(face),
        15..=20 => wave(face),
        21..=30 => hum(face),
        31..=40 => sit(face),
        41..=60 => sparkle(face),
        61..=68 => shimmer(face),
        69..=80 => float_sparkle(face),
        81..=90 => sing(face),
        _ => excite(face),
    }
}

fn random_play(face: &str) -> Animation {
    match rand_range(0,100) {
        0..=30 => play(face),
        31..=60 => punch(face),
        61..=80 => silly(face),
        81..=90 => sing(face),
        _ => magic(face)
    }
}

fn random_confused(face: &str) -> Animation {
    match rand_range(0,100) {
        0..=30 => sweat(face),
        _ => ponder(face)
    }
}

fn random_creative(face: &str) -> Animation {
    match rand_range(0,100) {
        0..=10 => magic(face),
        11..=30 => build(face),
        31..=36 => teleport(face),
        37..=54 => make(face),
        55..=60 => observe(face),
        81..=90 => sing(face),
        _ => ponder(face)
    }
}

fn random_cheeky(face: &str) -> Animation {
    match rand_range(0,100) {
        0..=10 => sit(face),
        11..=30 => look_around(face),
        31..=40 => silly(face),
        41..=50 => shimmer(face),
        51..=60 => table_flip(face),
        61..=70 => zombie(face),
        71..=80 => booty(face),
        _ => flex(face)
    }
}

fn random_content(face: &str) -> Animation {
    match rand_range(0,100) {
        0..=30 => sit(face),
        31..=70 => look_around(face),
        71..=85 => wave(face),
        _ => hum(face)
    }
}

fn random_curious(face: &str) -> Animation {
    match rand_range(0,100) {
        0..=30 => inspect(face),
        31..=50 => observe(face),
        _ => ponder(face)
    }
}

fn random_sad(face: &str) -> Animation {
    match rand_range(0,100) {
        0..=30 => big_cry(face),
        31..=45 => sweat(face),
        46..=70 => loaf(face),
        _ => cry(face)
    }
}

fn random_tired(face: &str) -> Animation {
    match rand_range(0,100) {
        0..=30 => sleep(face),
        _ => tired(face)
    }
}



fn sit(face: &str) -> Animation {
    Animation::new(&[
        Frame::new(format!("( {} )", face), rand_range(2000,5000)),
        Frame::new(format!(" ( {} ) ･", face), 300),
        Frame::new(format!("  ( {} ) ･･", face), 300),
        Frame::new(format!(" ( {} ) ･", face), 300),
        Frame::new(format!("( {} )", face), rand_range(1000,3500)),
        Frame::new(format!("･ ( {} )  ", face), 300),
        Frame::new(format!("･･ ( {} )   ", face), 300),
        Frame::new(format!("･ ( {} )   ", face), 300),
    ], rand_range(2,8))
}

fn look_around(face: &str) -> Animation {
    Animation::new(&[
        Frame::new(format!("( {} )", face), rand_range(3000,10000)),
        Frame::new(format!("(  {})", face), rand_range(250,1000)),
        Frame::new(format!("( {} )", face), rand_range(250,1000)),
        Frame::new(format!("({}  )", face), rand_range(250,1000)),
        Frame::new(format!("( {} )", face), rand_range(1500,5000)),
        Frame::new(format!("(  {})", face), rand_range(250,1000)),
        Frame::new(format!("( {} )", face), rand_range(250,1000)),
        Frame::new(format!("({}  )", face), rand_range(450,1000)),
    ], rand_range(2,7))
}

fn float_sparkle(face: &str) -> Animation {
    Animation::new(&[
        Frame::new(format!("( {} )", face), rand_range(2000,8000)),
        Frame::new(format!("☆( {} )☆", face), 350),
        Frame::new(format!("*( {} )*", face), 400),
        Frame::new(format!("'( {} )'", face), 500),
    ], rand_range(2,8))
}

fn sparkle(face: &str) -> Animation {
    Animation::new(&[
        Frame::new(format!("( {} )", face),  rand_range(2000,10000)),
        Frame::new(format!("¤( {} )¤", face), 200),
        Frame::new(format!("⊹( {} )⊹", face), 250),
    ], rand_range(2,8))
}

fn shimmer(face: &str) -> Animation {
    Animation::new(&[
        Frame::new(format!("( {} )", face),  rand_range(5000,10000)),
        Frame::new(format!("  ( {} ) ¤", face), 200),
        Frame::new(format!("  ( {} ) ⊹", face), 250),
        Frame::new(format!("( {} )", face),  rand_range(2000,10000)),
        Frame::new(format!("¤ ( {} )  ", face), 200),
        Frame::new(format!("⊹ ( {} )  ", face), 250),
    ], rand_range(2,8))
}

fn punch(_face: &str) -> Animation {
    Animation::new(&[
        Frame::new(format!(" O('-' Q)  "), 750),
        Frame::new(format!("O=('-' Q)  "), 250),
        Frame::new(format!(" O('-' Q)  "), 750),
        Frame::new(format!(" O('-'O=)  "), 250),
        Frame::new(format!(" O('-' Q)  "), rand_range(200,4000)),
    ], rand_range(2,5))
}

fn excite(face: &str) -> Animation {
    Animation::new(&[
        Frame::new(format!(" _( {} )_ ", face), rand_range(50,3000)),
        Frame::new(format!("~\\( {} )/~", face), 250),
        Frame::new(format!("* \\( {} )/ *", face), 250),
        Frame::new(format!("☆  \\( {} )/  ☆", face), 250),
        Frame::new(format!(".   \\( {} )/   .", face), 500),
    ], rand_range(2,5))
}

fn play(_face: &str) -> Animation {
    Animation::new(&[
        Frame::new(format!("     (  •_•)     "), rand_range(500,5000)),
        Frame::new(format!("     (  •_•)>⌐■-■"), 450),
        Frame::new(format!("     (  •_•)⌐■-■ "), 300),
        Frame::new(format!("     (  •⌐■)■    "), 300),
        Frame::new(format!("     ( ⌐■_■)     "), 3000),
        Frame::new(format!("     ( ⌐■‿■)     "), 1750),
    ], rand_range(1,2))
}

fn magic(_face: &str) -> Animation {
    Animation::new(&[
        Frame::new(format!("      (  •‿•)      "), rand_range(1000,6000)),
        Frame::new(format!("      (  •‿•)⌒     "), rand_range(500,3000)),
        Frame::new(format!("      (  •‿•)─☆    "), 200),
        Frame::new(format!("      (  •‿•)─ ☆   "), 200),
        Frame::new(format!("      (  •‿•)─  ☆  "), 200),
        Frame::new(format!("      (  •‿•)─   ✧ "), 200),
        Frame::new(format!("      (  •▽•)     §"), 330),
        Frame::new(format!("      (  •▽•)     ✧"), 330),
        Frame::new(format!("      (  •▽•)      "), 1000),
    ], rand_range(1,4))
}

fn give_love(face: &str) -> Animation {
    Animation::new(&[
        Frame::new(format!(" ( {} ) ", face), rand_range(500,5000)),
        Frame::new(format!(" (  {}) ", face), rand_range(1000,3000)),
        Frame::new(format!(" (  {})♥", face), 1000),
    ], rand_range(1,4))
}

fn sweat(face: &str) -> Animation {
    Animation::new(&[
        Frame::new(format!("( {} )", face),  rand_range(400,3000)),
        Frame::new(format!(":( {} ):", face), 250),
        Frame::new(format!(". ( {} ) .", face), 250),
    ], rand_range(3,8))
}

fn cry(face: &str) -> Animation {
    Animation::new(&[
        Frame::new(format!("( {} )", face),  rand_range(350,2000)),
        Frame::new(format!("-( {} )-", face), 250),
        Frame::new(format!(". ( {} ) .", face), 250),
    ], rand_range(3,6))
}

fn big_cry(_face: &str) -> Animation {
    Animation::new(&[
        Frame::new(format!("✧( >д< )✧"), 270),
        Frame::new(format!("ﾟ･( >д< )･ﾟ"), 300),
        Frame::new(format!("ﾟﾟ ( >д< ) ﾟﾟ"), 350),
        Frame::new(format!("✧ﾟ  ( >д< )  ﾟ✧"), 320),
        Frame::new(format!("｡   ( >д< )   ｡"), 250),
    ], rand_range(3,6))
}

fn sleep(face: &str) -> Animation {
    Animation::new(&[
        Frame::new(format!("( {} )", face),  rand_range(350,1200)),
        Frame::new(format!(" ( {} )z", face), 250),
        Frame::new(format!("  ( {} ) Z", face), 250),
        Frame::new(format!("   ( {} )  z", face), 250),
    ], rand_range(6,12))
}


fn tired(face: &str) -> Animation {
    Animation::new(&[
        Frame::new(format!("( {} )", face),  rand_range(1000,6000)),
        Frame::new(format!("✧( {} ) ", face), 350),
        Frame::new(format!("⁘( {} ) ", face), 350),
        Frame::new(format!("⁛( {} ) ", face), 350),
    ], rand_range(3,6))
}

fn flex(_face: &str) -> Animation {
    Animation::new(&[
        Frame::new(format!("ᕦ(⇀‸↼‶ )ᕥ"), 1500),
        Frame::new(format!("ᕙ(⇀‸↼ ‶)ᕗ"), 1500),
    ], rand_range(2,7))
}

fn ponder(face: &str) -> Animation {
    Animation::new(&[
        Frame::new(format!("(  {})", face),  rand_range(500,4000)),
        Frame::new(format!(" (  {})?", face), 250),
        Frame::new(format!("  (  {}) ?", face), 250),
        Frame::new(format!("(  {})", face), rand_range(500,4000)),
        Frame::new(format!("  (  {}) ･", face), 250),
        Frame::new(format!("   (  {}) ･･", face), 250),
        Frame::new(format!("    (  {}) ･･･", face), 500),
        Frame::new(format!("    (  {})  ･･", face), 250),
        Frame::new(format!("    (  {})   ･", face), 250),
    ], rand_range(3,7))
}

fn inspect(face: &str) -> Animation {
    Animation::new(&[
        Frame::new(format!("  (  {}) ▄", face), rand_range(2000,5000)),
        Frame::new(format!("  (  {})_▄", face), rand_range(500,2000)),
        Frame::new(format!("  (  {})─■", face), rand_range(1000,3000)),
        Frame::new(format!(" ?(  {})─■", face), rand_range(250,500)),
        Frame::new(format!("? (  {})─■", face), rand_range(250,500)),
        Frame::new(format!("  (  {})─■", face), rand_range(1000,2000)),
        Frame::new(format!("  (  {})_▄", face), rand_range(500,1000)),
    ], rand_range(3,7))
}

fn observe(face: &str) -> Animation {
    Animation::new(&[
        Frame::new(format!("    (  {})  ╭▀╮", face), rand_range(3000,9000)),
        Frame::new(format!("     (  {}) °╭─╮°", face), 200),
        Frame::new(format!("      (  {})° ╭─╮ °", face), 200),
        Frame::new(format!("    (  {})  ╭─╮", face), rand_range(2000,5000)),
        Frame::new(format!("      (  {})° ╭─╮ °", face), 200),
        Frame::new(format!("     (  {}) °╭─╮°", face), 200),
    ], rand_range(3,7))
}

fn sing(face: &str) -> Animation {
    Animation::new(&[
        Frame::new(format!("  (  {}) ♬", face), 500),
        Frame::new(format!("  (  {})♪ ", face), 500),
        Frame::new(format!(" ♪(  {}) ♪", face), 500),
        Frame::new(format!("♪ (  {})  ", face), 500),
        Frame::new(format!("  (  {})♬ ", face), 500),
    ], rand_range(3,7))
}

fn endure(_face: &str) -> Animation {
    Animation::new(&[
        Frame::new(format!("(\"◡_◡ )"), 500),
        Frame::new(format!("(✧◡_◡ )"), 500),
        Frame::new(format!("(.◡_◡ )"), 500),
        Frame::new(format!("( ◡_◡ )"), rand_range(500, 4000)),
    ], rand_range(3,8))
}

fn build(_face: &str) -> Animation {
    Animation::new(&[
        Frame::new(format!("  (  •‿•)  "), rand_range(500,6000)),
        Frame::new(format!("  (  •‿•)⌒■"), 500),
        Frame::new(format!("  (  •‿•)─▄"), 500),
        Frame::new(format!("  (  •‿•)⌒▄"), 500),
        Frame::new(format!("  (  •‿•)─█"), 500),
        Frame::new(format!("  (  •‿•) █"), rand_range(500,2000)),
        Frame::new(format!("  (  •▽•) █"), rand_range(1000,5000)),
        Frame::new(format!("  (  •‿•)⌒█"), 500),
        Frame::new(format!("   (  •‿•)─▄▀"), 250),
        Frame::new(format!("   (  •‿•)─▄▄"), 500),
        Frame::new(format!("   (  •‿•) ▄▄"), rand_range(1000,5000)),
    ], rand_range(2,5))
}

fn teleport(face: &str) -> Animation {
    Animation::new(&[
        Frame::new(format!("({}  )", face), rand_range(1000,6000)),
        Frame::new(format!("✧   ({}  )     ", face), 20),
        Frame::new(format!("☆   ({}  )     ", face), 200),
        Frame::new(format!("☆▒☆  ({}  )      ", face), 200),
        Frame::new(format!("✧ █ ✧ ({}  )       ", face), 200),
        Frame::new(format!("█   ({}  )     ", face), rand_range(750,5000)),
        Frame::new(format!("☆   ({}  )     ", face), 200),
    ], rand_range(2,5))
}

fn hum(face: &str) -> Animation {
    Animation::new(&[
        Frame::new(format!("  ( {} )♪ ", face), 500),
        Frame::new(format!("  ( {} ) ♪", face), 500),
        Frame::new(format!(" ♪( {} )  ", face), 500),
        Frame::new(format!("♪ ( {} )  ", face), 500),
        Frame::new(format!("  ( {} )  ", face), rand_range(500,2000)),
    ], rand_range(4,10))
}

fn wave(face: &str) -> Animation {
    Animation::new(&[
        Frame::new(format!(" ( {} )ﾉ", face), 500),
        Frame::new(format!(" ( {} )/", face), 500),
    ], rand_range(4,10))
}

fn silly(face: &str) -> Animation {
    Animation::new(&[
        Frame::new(format!("┌( {} )╯", face), 500),
        Frame::new(format!("└( {} )┐", face), 500),
    ], rand_range(4,10))
}

fn table_flip(face: &str) -> Animation {
    Animation::new(&[
        Frame::new(format!("    ( {} ) ┬──┬", face), rand_range(500,6000)),
        Frame::new(format!("    (  {}) ┬──┬", face), rand_range(1000,7000)),
        Frame::new(format!("    (  `Д´)ﾉ┬──┬"), 250),
        Frame::new(format!("      (  `Д´)ﾉ  ┴──┴"), 250),
        Frame::new(format!("       (  `Д´)    ┴──┴"), 300),
        Frame::new(format!("         (  {})     ┬──┬", face), rand_range(1000,6000)),
    ], rand_range(1,2))
}

fn make(face: &str) -> Animation {
    Animation::new(&[
        Frame::new(format!("┬─┬ ({}  )    ", face), rand_range(3000,10000)),
        Frame::new(format!("┬─┬⊂({}  )    ", face), rand_range(300,3000)),
        Frame::new(format!("┬÷┬⊂({}  )    ", face), 400),
        Frame::new(format!("┬=┬⊂({}  )    ", face), 350),
        Frame::new(format!("┬÷┬⊂({}  )    ", face), 350),
        Frame::new(format!("┬=┬⊂({}\" )    ", face), 350),
        Frame::new(format!("┬÷┬⊂({}: )    ", face), 350),
        Frame::new(format!("┬=┬⊂({}, )    ", face), 350),
        Frame::new(format!("┬÷┬⊂({}  )    ", face), 350),
        Frame::new(format!("┬=┬⊂({}  )    ", face), 350),
        Frame::new(format!("☆┬▀┬ ({}  )     ", face), 250),
        Frame::new(format!("* ┬▀┬ ({}  )      ", face), 250),
        Frame::new(format!("┬▀┬ ({}  )    ", face), rand_range(2000,8000)),
    ], rand_range(3,7))
}

fn zombie(_face: &str) -> Animation {
    Animation::new(&[
        Frame::new(format!(" ( ¬º-°)¬"), rand_range(500,2000)),
        Frame::new(format!(" ( ¬°-º)¬"), rand_range(500,2000)),
    ], rand_range(4,9))
}

fn loaf(face: &str) -> Animation {
    Animation::new(&[
        Frame::new(format!("/ {} \\", face), rand_range(1000,6000)),
    ], rand_range(4,9))
}

fn booty(_face: &str) -> Animation {
    Animation::new(&[
        Frame::new(format!("( ‿|‿ )"), rand_range(700, 1000)),
        Frame::new(format!("(‿(‿  )"), rand_range(300, 800)),
        Frame::new(format!("( ‿|‿ )"), rand_range(300, 500)),
        Frame::new(format!("(  ‿)‿)"), rand_range(300, 800)),
    ], rand_range(2,4))
}

fn eat(face: &str) -> Animation {
    Animation::new(&[
        Frame::new(format!("  ( {} ) ▀", face), 250),
        Frame::new(format!("  ( {} ) ▄", face), 250),
        Frame::new(format!(" !(  {}) ▄", face), 250),
        Frame::new(format!("! (  {}) ▄", face), 250),
        Frame::new(format!("  (  {}) ▄", face), rand_range(2000,5000)),
        Frame::new(format!("  (  {})~▄", face), 250),
        Frame::new(format!(" (  ˘□˘)▄"), 250),
        Frame::new(format!("(   ˘▄)"), 250),
        Frame::new(format!("(  ˘ω˘)"), 500),
        Frame::new(format!("(  ˘3˘)"), 500),
        Frame::new(format!("(  ˘ω˘)"), 500),
        Frame::new(format!("(  ˘3˘)"), 500),
        Frame::new(format!("¤(  ˘ω˘)¤"), 500),
        Frame::new(format!("⊹(  ˘3˘)⊹"), 500),
        Frame::new(format!("(  ˘ω˘)"), 500),
        Frame::new(format!("(  ˘3˘)"), 500),
        Frame::new(format!("(  ˘ڡ˘)"), rand_range(1000,8000)),
    ], 1)
}
