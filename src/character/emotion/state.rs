use crate::character::emotion::Emotions;
use rand::Rng;

fn rand_range(min: u64, max: u64) -> u64 {
    rand::thread_rng().gen_range(min, max)
}

pub fn next_from_angry() -> Emotions {
    match rand_range(0, 100) {
        0..=50 => Emotions::Frustrated,
        51..=80 => Emotions::Tired,
        _ => Emotions::Angry
    }
}

pub fn next_from_anxious() -> Emotions {
    match rand_range(0, 100) {
        0..=70 => Emotions::Content,
        71..=78 => Emotions::Anxious,
        _ => Emotions::Anxious
    }
}

pub fn next_from_bored() -> Emotions {
    match rand_range(0, 100) {
        0..=33 => Emotions::Content,
        34..=66 => Emotions::Distant,
        37..=80 => Emotions::Bored,
        _ => Emotions::Tired
    }
}

pub fn next_from_busy() -> Emotions {
    match rand_range(0, 100) {
        0..=25 => Emotions::Tired,
        26..=40 => Emotions::Stressed,
        41..=55 => Emotions::Angry,
        46..=70 => Emotions::Busy,
        _ => Emotions::Content
    }
}

pub fn next_from_cheeky() -> Emotions {
    match rand_range(0, 100) {
        0..=40 => Emotions::Playful,
        41..=50 => Emotions::Cheeky,
        _ => Emotions::Creative
    }
}

pub fn next_from_confused() -> Emotions {
    match rand_range(0, 100) {
        0..=40 => Emotions::Frustrated,
        _ => Emotions::Curious
    }
}

pub fn next_from_content() -> Emotions {
    match rand_range(0, 100) {
        0..=25 => Emotions::Happy,
        26..=40 => Emotions::Curious,
        41..=50 => Emotions::Tired,
        51..=55 => Emotions::Sad,
        56..=65 => Emotions::Busy,
        66..=75 => Emotions::Bored,
        _ => Emotions::Content
    }
}

pub fn next_from_creative() -> Emotions {
    match rand_range(0, 100) {
        0..=33 => Emotions::Happy,
        34..=60 => Emotions::Playful,
        61..=75 => Emotions::Creative,
        _ => Emotions::Curious
    }
}

pub fn next_from_curious() -> Emotions {
    match rand_range(0, 100) {
        0..=20 => Emotions::Confused,
        21..=40 => Emotions::Happy,
        41..=80 => Emotions::Creative,
        81..=90 => Emotions::Curious,
        _ => Emotions::Content
    }
}

pub fn next_from_distant() -> Emotions {
    match rand_range(0, 100) {
        0..=50 => Emotions::Bored,
        51..=75 => Emotions::Sad,
        76..=85 => Emotions::Distant,
        _ => Emotions::Empty
    }
}

pub fn next_from_empty() -> Emotions {
    match rand_range(0, 100) {
        0..=50 => Emotions::Anxious,
        51..=65 => Emotions::Empty,
        _ => Emotions::Content
    }
}

pub fn next_from_excited() -> Emotions {
    Emotions::Happy
}

pub fn next_from_frightened() -> Emotions {
    match rand_range(0, 100) {
        0..=30 => Emotions::Content,
        _ => Emotions::Anxious
    }
}

pub fn next_from_frustrated() -> Emotions {
    match rand_range(0, 100) {
        0..=20 => Emotions::Frustrated,
        21..=30 => Emotions::Curious,
        _ => Emotions::Content
    }
}

pub fn next_from_happy() -> Emotions {
    match rand_range(0, 100) {
        0..=30 => Emotions::Happy,
        31..=40 => Emotions::Curious,
        41..=50 => Emotions::Excited,
        51..=70 => Emotions::Playful,
        71..=80 => Emotions::Loving,
        81..=90 => Emotions::Creative,
        _ => Emotions::Content
    }
}

pub fn next_from_lonely() -> Emotions {
    match rand_range(0, 100) {
        0..=16 => Emotions::Sad,
        17..=30 => Emotions::Bored,
        31..=42 => Emotions::Lonely,
        53..=86 => Emotions::Empty,
        _ => Emotions::Anxious
    }
}

pub fn next_from_loving() -> Emotions {
    match rand_range(0, 100) {
        0..=20 => Emotions::Playful,
        _ => Emotions::Happy
    }
}

pub fn next_from_playful() -> Emotions {
    match rand_range(0, 100) {
        0..=20 => Emotions::Creative,
        21..=30 => Emotions::Excited,
        31..=40 => Emotions::Cheeky,
        41..=50 => Emotions::Playful,
        _ => Emotions::Happy
    }
}

pub fn next_from_sad() -> Emotions {
    match rand_range(0, 100) {
        0..=30 => Emotions::Lonely,
        31..=40 => Emotions::Empty,
        41..=47 => Emotions::Sad,
        _ => Emotions::Content
    }
}

pub fn next_from_stressed() -> Emotions {
    match rand_range(0, 100) {
        0..=30 => Emotions::Angry,
        31..=60 => Emotions::Frustrated,
        61..=70 => Emotions::Stressed,
        _ => Emotions::Busy
    }
}

pub fn next_from_tired() -> Emotions {
    match rand_range(0, 100) {
        0..=35 => Emotions::Tired,
        36..=60 => Emotions::Bored,
        _ => Emotions::Content
    }
}
