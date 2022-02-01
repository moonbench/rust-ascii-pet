use rand::seq::SliceRandom;

#[derive(Debug, Clone)]
pub enum Emotions {
    Happy,
    Excited,
    Loving,
    Sad,
    Playful,
    Null
}

fn happy_face() -> &'static str {
    let faces = vec![
        "^‿^",
        "^_^",
        "ˊᵕˋ",
        "•‿•",
        "˘⌣˘",
        "˘◡˘",

    ];
    faces.choose(&mut rand::thread_rng()).unwrap()
}

fn excited_face() -> &'static str {
    let faces = vec![
        "^-^",
        "^O^",
        "⌒▽⌒",

    ];
    faces.choose(&mut rand::thread_rng()).unwrap()
}

fn loving_face() -> &'static str {
    let faces = vec![
        "♡‿♡",
        "´ω`",
        "˘³˘",
    ];
    faces.choose(&mut rand::thread_rng()).unwrap()
}

#[derive(Debug)]
pub struct Emotion {
    pub name: Emotions,
}

impl Emotion {
    pub fn pick_expression(&self) -> String {
        match self.name {
            Emotions::Happy => happy_face(),
            Emotions::Excited => excited_face(),
            Emotions::Loving => loving_face(),
            Emotions::Sad => "╥^╥",
            Emotions::Null => "o.o",
            Emotions::Playful => ":3:"
        }.to_string()
    }
}
