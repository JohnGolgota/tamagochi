use rand::prelude::*;
#[derive(Debug, PartialEq, Eq)]
pub enum PetStatus {
    Alive,
    Sick,
    Death,
}

pub struct Pet {
    pub name: String,
    pub status: PetStatus,
    pub stats: Stats,
    // pub happy: u8
}

impl Pet {
    pub fn new(name: String) -> Pet {
        Pet {
            name,
            status: PetStatus::Alive,
            stats: Stats::new(),
        }
    }
    pub fn check_status(&mut self) {
        // donde deberia estar el tick en esto o en el loop arriba?
        let health = self.stats.health; // no
        if health == 0 {
            // self.status = String::from("death"); // no necesita returns
            self.status = PetStatus::Death;
        } else if health > 0 && health < 50 {
            // self.status = String::from("sick"); // no necesita returns
            self.status = PetStatus::Sick;
        } else {
            // self.status = String::from("Alive"); // *carita sonrojada*
            self.status = PetStatus::Alive;
        }
    }
    pub fn live(&mut self) {
        // da hunger sip esa es la idea
		self.check_status();
        self.stats.tick(); // tenes copilot o que es lo que auto completa?
    }
    pub fn feed(&mut self) {
        self.stats.feed();
    }
}

pub struct Stats {
    pub health: u8,
    pub hunger: u8,
    pub boredom: u8,
    pub tiredness: u8,
    pub dirtiness: u8,
}

impl Stats {
    fn new() -> Stats {
        Stats {
            health: 10,
            hunger: 0,
            boredom: 0,
            tiredness: 0,
            dirtiness: 0,
        }
    }
    fn feed(&mut self) {
        self.hunger -= 5;
    }
    fn play(&mut self) {
        self.boredom -= 1;
    }
    fn sleep(&mut self) {
        self.tiredness -= 1;
    }
    fn wash(&mut self) {
        self.dirtiness -= 1;
    }
    fn tick(&mut self) {
        if self.health > 0 {
            self.health -= 1;
            let num = randnum();
            if num % 2u8 == 0u8 {
                self.hunger += 1u8;
                if num > 5u8 {
                    self.tiredness += 1u8;
                }
            }
            if num % 3u8 == 0u8 {
                self.boredom += 1u8;
            }
            self.dirtiness += 1u8;
            // self.hunger += 1;
        }
    }
    fn is_alive(&self) -> bool {
        self.hunger < 100 && self.boredom < 100 && self.tiredness < 100 && self.dirtiness < 100
    }
    // fn is_happy(&mut self) -> bool {
    //     self.happy =
    //         self.hunger < 50 && self.boredom < 50 && self.tiredness < 50 && self.dirtiness < 50;
    //     self.happy
    // }
    fn is_hungry(&self) -> bool {
        self.hunger > 50
    }
    fn is_bored(&self) -> bool {
        self.boredom > 50
    }
    fn is_tired(&self) -> bool {
        self.tiredness > 50
    }
    fn is_dirty(&self) -> bool {
        self.dirtiness > 50
    }
    fn is_dead(&self) -> bool {
        self.hunger >= 100 || self.boredom >= 100 || self.tiredness >= 100 || self.dirtiness >= 100
    }
}

fn randnum() -> u8 {
    let rand_number: u8 = rand::thread_rng().gen_range(1..=10);
    rand_number
}
