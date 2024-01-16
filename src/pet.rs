use rand::prelude::*;
#[derive(Debug, PartialEq, Eq)]
pub enum PetStatus {
    Alive,
    Sick,
    Death,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CatSize {
    Small {
        width:usize, 
        height:usize
    },
    Medium {
        width:usize, 
        height:usize
    },
    Big {
        width:usize,
        height:usize
    }
}

impl CatSize{
    pub fn new() -> CatSize{
        CatSize::Medium {
            width: 12, 
            height: 6 
        }
        
    }
}

impl Default for CatSize {
    fn default() -> Self {
        CatSize::new()
    }
}

pub struct Pet {
    pub name: String,
    pub status: PetStatus,
    pub stats: Stats,
    pub size: CatSize
}

impl Pet {
    pub fn new(name: String) -> Pet {
        Pet {
            name,
            status: PetStatus::Alive,
            stats: Stats::new(),
            size: CatSize::default()
        }
    }
    pub fn check_status(&mut self) {
        let health = self.stats.health;
        if health == 0 || self.stats.is_dead() {
            self.status = PetStatus::Death;
        } else if health > 0 && health < 50 {
            self.status = PetStatus::Sick;
        } else {
            self.status = PetStatus::Alive;
        }
    }
    pub fn live(&mut self) {
        self.check_status();
        self.stats.tick();
    }
    pub fn feed(&mut self) {
        self.stats.feed();
    }
    pub fn play(&mut self) {
        self.stats.play();
    }
    pub fn sleep(&mut self) {
        self.stats.sleep();
    }
    pub fn wash(&mut self) {
        self.stats.wash();
		println!("{} se ha lavado", self.name)
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
        if self.hunger >= 5u8 {
            self.hunger -= 5u8;
        }
    }
    fn play(&mut self) {
        if self.boredom >= 5u8 {
            self.boredom -= 5u8;
        }
    }
    fn sleep(&mut self) {
        if self.tiredness >= 5u8 {
            self.tiredness -= 5u8;
        }
    }
    fn wash(&mut self) {
        if self.dirtiness >= 5u8 {
            self.dirtiness -= 5u8;
        }
    }
    fn tick(&mut self) {
        if self.health > 0 {
            if self.is_bored() || self.is_tired() || self.is_dirty() || self.is_hungry() {
                self.health -= 1;
            }
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
        }
    }
    fn _is_alive(&self) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_num() {
        let num = randnum();
        assert!(num > 0 && num <= 10);
    }
    #[test]
    fn test_stats() {
        let mut stats = Stats::new();
        stats.tick();
        assert_eq!(stats.health, 10);
        assert!(stats.hunger == 0 || stats.hunger == 1);
        assert!(stats.boredom == 0 || stats.boredom == 1);
        assert!(stats.tiredness == 0 || stats.tiredness == 1);
        assert!(stats.dirtiness == 0 || stats.dirtiness == 1);
        let mut stats = Stats::new();
        loop {
            stats.tick();
            if stats.hunger == 5 {
                break;
            }
        }
        stats.feed();
        assert_eq!(stats.hunger, 0);
        let mut stats = Stats::new();
        loop {
            stats.tick();
            if stats.boredom == 5 {
                break;
            }
        }
        stats.play();
        assert_eq!(stats.boredom, 0);
        let mut stats = Stats::new();
        loop {
            stats.tick();
            if stats.tiredness == 5 {
                break;
            }
        }
        stats.sleep();
        assert_eq!(stats.tiredness, 0);
        let mut stats = Stats::new();
        loop {
            stats.tick();
            if stats.dirtiness == 5 {
                break;
            }
        }
        stats.wash();
        assert_eq!(stats.dirtiness, 0);
    }
}
