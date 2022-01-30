use rand::seq::SliceRandom;

#[derive(Debug)]
pub enum Emotions {
    Happy,
    Excited,
    Loving,
    Sad,
    Null
}

fn happy_face() -> &'static str {
    let faces = vec![
        "^‿^",
        "^_^",
        "ˊᵕˋ"
    ];
    faces.choose(&mut rand::thread_rng()).unwrap()
}

#[derive(Debug)]
pub struct Emotion<'a> {
    pub name: &'a Emotions,
}

impl Emotion<'_> {
    pub fn pick_expression(&self) -> String {
        match self.name {
            Emotions::Happy => happy_face(),
            Emotions::Excited => "≧▽≦",
            Emotions::Loving => "♥‿♥",
            Emotions::Sad => "╥﹏╥",
            Emotions::Null => "o.o",
        }.to_string()
    }
}
