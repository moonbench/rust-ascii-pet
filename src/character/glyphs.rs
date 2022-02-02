use rand::seq::SliceRandom;

pub fn happy_faces() -> Vec<&'static str> {
    vec![
        "^‿^",
        "^_^",
        "ˊᵕˋ",
        "•‿•",
        "˘⌣˘",
        "˘◡˘"
    ]
}
pub fn loving_faces() -> Vec<&'static str> {
    vec![
        "♡‿♡",
        "´ω`",
        "˘³˘",
    ]
}
pub fn excited_faces() -> Vec<&'static str> {
    vec![
        "^-^",
        "⌒▽⌒",
        "•ᴥ•"
    ]
}
pub fn sad_faces() -> Vec<&'static str> {
    vec![
        "╥˷╥",
        "╥_╥",
        "╥ ╥",
    ]
}
pub fn null_face() -> &'static str {
    "•.•"
}
pub fn playful_face() -> &'static str {
    ":3:"
}

pub fn default_ears() -> &'static str {
    "ᑎ___ᑎ"
}
pub fn sport_ears() -> &'static str {
    "(\\___/)"
}
pub fn small_ears() -> &'static str {
    "೧___೧"
}


pub fn random_happy_face() -> &'static str {
    happy_faces().choose(&mut rand::thread_rng()).unwrap()
}
pub fn random_excited_face() -> &'static str {
    excited_faces().choose(&mut rand::thread_rng()).unwrap()
}
pub fn random_loving_face() -> &'static str {
    loving_faces().choose(&mut rand::thread_rng()).unwrap()
}
pub fn random_sad_face() -> &'static str {
    sad_faces().choose(&mut rand::thread_rng()).unwrap()
}
