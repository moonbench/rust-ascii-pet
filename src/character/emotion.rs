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
            Emotions::Excited => "^-^",
            Emotions::Loving => "♥‿♥",
            Emotions::Sad => "╥^╥",
            Emotions::Null => "o.o",
            Emotions::Playful => ":3:"
        }.to_string()
    }
}
