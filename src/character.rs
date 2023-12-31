pub mod emotion;
pub mod animation;
pub mod glyphs;
pub mod vitals;

use crate::character::emotion::{Emotion, Emotions};
use crate::character::animation::{Animation};
use crate::character::vitals::Vitals;

use std::io::{Write, stdout};
use crate::draw;
use crossterm::{
    execute,
    style::{Color, SetForegroundColor},
};

#[derive(Debug)]
pub struct Character {
    pub name: String,
    pub face: String,
    pub ears: String,
    pub emotion: Emotion,
    pub animation: Animation,
    pub vitals: Vitals,
    pub wealth: u32,
    pub position: (u8, u8),
}

impl Character  {
    pub fn default() -> Character {
        let emotion = Emotion {
            name: Emotions::Content
        };
        let vitals = Vitals::default();
        Character {
            name: "Buddy".to_string(),
            face: emotion.pick_expression(),
            ears: "ᑎ___ᑎ".to_string(),
            animation: Animation::make_for(&emotion, &emotion.pick_expression()),
            emotion: emotion,
            vitals: vitals,
            wealth: 100,
            position: (8, 0),
        }
    }

    pub fn next_tick(&mut self) -> u64 {
        execute!(
            std::io::stdout(),
            SetForegroundColor(Color::White),
        ).unwrap();
        let x_pos = self.position.0 as u16 + 1;
        self::draw(&format!("{:^24}", &self.ears), (x_pos, 7));
        self::draw(&format!("{:^24}", &self.animation.frame().0), (x_pos, 8));
        let _ = stdout().flush();
        let delay = self.animation.frame().1;
        self.animation.next();
        delay
    }

    pub fn set_emotion(&mut self, name: Emotions){
        self.emotion = Emotion{name: name.clone()};
        self.face = self.emotion.pick_expression();
        self.ears = self.emotion.pick_ears();
        self.animation = Animation::make_for(&self.emotion, &self.face);
        self.update_vitals();
    }

    pub fn state_change(&mut self) {
        self.set_emotion(self.emotion.next_emotion(&self.vitals));
    }

    pub fn set_state(&mut self, new_emotion: Emotions) {
        self.set_emotion(new_emotion);
    }

    pub fn can_feed(&self) -> bool {
        if self.wealth < 10 { return false; }
        if self.emotion.name == Emotions::Eating { return false; }
        true
    }

    pub fn feed(&mut self) {
        self.decrease_wealth(10);
        self.set_state(Emotions::Eating);
    }

    fn increase_wealth(&mut self, addition: u32) {
        self.wealth += addition;
        if self.wealth > 10000 { self.wealth = 10000; }
    }

    fn decrease_wealth(&mut self, subtraction: u32) {
        if self.wealth + subtraction > 0 {
            self.wealth -= subtraction;
        }
    }


    fn update_vitals(&mut self) {
        match self.emotion.name {
            Emotions::Angry => {
                self.vitals.less_politeness();
                self.vitals.less_energy();
            },
            Emotions::Anxious => self.vitals.less_confident(),
            Emotions::Bored => {
                self.vitals.less_strength();
                self.vitals.less_engagement();
                self.vitals.more_relaxation();
                if self.vitals.happiness > 60 { self.vitals.less_happiness(); }
             },
            Emotions::Busy => {
                self.vitals.less_happiness();
                self.increase_wealth(25);
            },
            Emotions::Cheeky => {
                self.vitals.more_brave();
                self.vitals.less_politeness();
            }
            Emotions::Confused => self.vitals.less_engagement(),
            Emotions::Content => {
                self.vitals.more_relaxation();
                self.vitals.less_engagement();
            }
            Emotions::Creative => {
                self.vitals.more_confident();
                self.increase_wealth(2);
            },
            Emotions::Curious => {
                self.vitals.more_engagement();
                self.vitals.less_relaxation();
            }
            Emotions::Distant => {
                self.vitals.less_engagement();
                if self.vitals.happiness > 60 { self.vitals.less_happiness(); }
            },
            Emotions::Eating => {
                self.vitals.more_energy();
                self.vitals.less_hunger();
                self.vitals.more_happiness();
            }
            Emotions::Empty => {
                self.vitals.less_engagement();
                self.vitals.less_happiness();
            },
            Emotions::Excited => {
                self.vitals.more_happiness();
            },
            Emotions::Frightened => self.vitals.less_brave(),
            Emotions::Frustrated => {
                self.vitals.less_relaxation();
                self.vitals.less_engagement();
             },
            Emotions::Happy => {
                self.vitals.more_happiness();
            },
            Emotions::Lonely => self.vitals.less_love(),
            Emotions::Loving => {
                self.vitals.more_love();
                self.vitals.more_politeness();
            },
            Emotions::Playful => {
                self.vitals.more_strength();
                self.vitals.more_hunger();
                self.vitals.more_brave();
                if self.vitals.energy > 5 { self.vitals.less_energy(); }
            },
            Emotions::Sad => {
                self.vitals.less_happiness();
                self.vitals.less_relaxation();
            },
            Emotions::Stressed => self.vitals.less_relaxation(),
            Emotions::Tired => {
                self.vitals.less_strength();
                self.vitals.more_energy();
                self.vitals.more_hunger();
            }
        }
    }
}
