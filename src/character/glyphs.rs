use rand::seq::SliceRandom;

pub fn happy_faces() -> Vec<&'static str> {
    vec![
        "^‿^",
        "^_^",
        "ˊᵕˋ",
        "•‿•",
        "•◡•",
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
        "^з^"
    ]
}
pub fn sad_faces() -> Vec<&'static str> {
    vec![
        "╥˷╥",
        "╥_╥",
        "╥ ╥",
    ]
}
pub fn cheeky_faces() -> Vec<&'static str> {
    vec![
        "¬‿¬",
        "˘³˘",
        "≖‿≖"
    ]
}
pub fn curious_faces() -> Vec<&'static str> {
    vec![
        "⇀‸↼",
        "ò_ô",
        "~_~",
        "ಠ_ಠ",
    ]
}
pub fn content_faces() -> Vec<&'static str> {
    vec![
        "-‿-",
        "˘⌣˘",
        "˘.˘",
        "-.-",
        "•⌣•",
        "^з^"
    ]
}
pub fn frightened_faces() -> Vec<&'static str> {
    vec![
        "⊙_⊙",
        "•⌂•",
        "°□°"
    ]
}

pub fn creative_faces() -> Vec<&'static str> {
    vec![
        "⇀‸↼",
        "˘⌣˘"
    ]
}

pub fn angry_face() -> &'static str {
    "`д´"
}
pub fn anxious_face() -> &'static str {
    "°^°"
}
pub fn bored_face() -> &'static str {
    "-_-"
}
pub fn busy_face() -> &'static str {
    "._."
}
pub fn frustrated_face() -> &'static str {
    "=_="
}
pub fn null_face() -> &'static str {
    "•.•"
}
pub fn playful_face() -> &'static str {
    "^з^"
}
pub fn tired_face() -> &'static str {
    "⇀‸↼"
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

pub fn random_cheeky_face() -> &'static str {
    cheeky_faces().choose(&mut rand::thread_rng()).unwrap()
}
pub fn random_content_face() -> &'static str {
    content_faces().choose(&mut rand::thread_rng()).unwrap()
}
pub fn random_creative_face() -> &'static str {
    creative_faces().choose(&mut rand::thread_rng()).unwrap()
}
pub fn random_curious_face() -> &'static str {
    curious_faces().choose(&mut rand::thread_rng()).unwrap()
}
pub fn random_excited_face() -> &'static str {
    excited_faces().choose(&mut rand::thread_rng()).unwrap()
}
pub fn random_frightened_face() -> &'static str {
    frightened_faces().choose(&mut rand::thread_rng()).unwrap()
}
pub fn random_happy_face() -> &'static str {
    happy_faces().choose(&mut rand::thread_rng()).unwrap()
}
pub fn random_loving_face() -> &'static str {
    loving_faces().choose(&mut rand::thread_rng()).unwrap()
}
pub fn random_sad_face() -> &'static str {
    sad_faces().choose(&mut rand::thread_rng()).unwrap()
}
