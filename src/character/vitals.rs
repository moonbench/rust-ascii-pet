#[derive(Debug)]
pub struct Vitals {
    pub bravery: i8,
    pub confidence: i8,
    pub energy: i8,
    pub engagement: i8,
    pub happiness: i8,
    pub love: i8,
    pub politeness: i8,
    pub relaxation: i8,
    pub strength: i8
}

impl Vitals  {
    pub fn default() -> Vitals {
        Vitals {
            bravery: 0,
            confidence: 0,
            energy: 0,
            engagement: 0,
            happiness: 0,
            love: 0,
            politeness: 0,
            relaxation: 0,
            strength: 0
        }
    }
    pub fn more_brave(&mut self) {
        if self.bravery > 110 { self.bravery -= 10 }
        self.bravery += 1;
    }

    pub fn less_brave(&mut self) {
        if self.bravery < -110 { self.bravery += 10 }
        self.bravery -= 1;
    }

    pub fn more_confident(&mut self) {
        if self.confidence > 110 { self.confidence -= 10 }
        self.confidence += 1;
    }

    pub fn less_confident(&mut self) {
        if self.confidence < -110 { self.confidence += 10 }
        self.confidence -= 1;
    }

    pub fn more_energy(&mut self) {
        if self.energy > 110 { self.energy -= 10 }
        self.energy += 1;
    }

    pub fn less_energy(&mut self) {
        if self.energy < -110 { self.energy += 10 }
        self.energy -= 1;
    }

    pub fn more_engagement(&mut self) {
        if self.engagement > 110 { self.engagement -= 10 }
        self.engagement += 1;
    }

    pub fn less_engagement(&mut self) {
        if self.engagement < -110 { self.engagement += 10 }
        self.engagement -= 1;
    }

    pub fn more_happiness(&mut self) {
        if self.engagement > 110 { self.engagement -= 10 }
        self.engagement += 1;
    }

    pub fn less_happiness(&mut self) {
        if self.engagement < -110 { self.engagement += 10 }
        self.engagement -= 1;
    }

    pub fn more_love(&mut self) {
        if self.love > 110 { self.love -= 10 }
        self.love += 1;
    }

    pub fn less_love(&mut self) {
        if self.love > 110 { self.love -= 10 }
        self.love -= 1;
    }

    pub fn more_politeness(&mut self) {
        if self.politeness > 110 { self.politeness -= 10 }
        self.politeness += 1;
    }

    pub fn less_politeness(&mut self) {
        if self.politeness < -110 { self.politeness += 10 }
        self.politeness -= 1;
    }

    pub fn more_relaxation(&mut self) {
        if self.relaxation > 110 { self.relaxation -= 10 }
        self.relaxation += 1;
    }

    pub fn less_relaxation(&mut self) {
        if self.relaxation < -110 { self.relaxation += 10 }
        self.relaxation -= 1;
    }

    pub fn more_strength(&mut self) {
        if self.strength > 110 { self.strength -= 10 }
        self.strength += 1;
    }

    pub fn less_strength(&mut self) {
        if self.engagement < -110 { self.strength += 10 }
        self.strength -= 1;
    }
}
