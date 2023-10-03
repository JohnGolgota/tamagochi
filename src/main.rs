use std::{thread, time};
use rand::Rng;

fn main() {
	let mut rng = rand::thread_rng();
    let mut mascota = Pet::new();
    let frames = [
        r#"
        /\_/\
       / o o \
       \¨ ^ ¨/
        /   \   \
       /|_|_|\__/
    "#,
        r#"
        /\_/\
       / = = \
       \¨ ^ ¨/
        /   \    _
       /|_|_|\__/
    "#,
        r#"
        /\_/\
       / o o \
       \¨ ^ ¨/
        /   \    _
       /|_|_|\__/
    "#,
        r#"
        /\_/\
       / = = \
       \¨ w ¨/
        /   \   \
       /|_|_|\__/
	"#,
        r#"
        /\_/\
       / o o \
       \¨ w ¨/
        /   \    _
       /|_|_|\__/
	"#,
    ];
    const MILLIS: u64 = 600;
    let mut msg = String::from("meaw");

    for frame in frames.iter().cycle() {
		let number:u8 = rng.gen_range(0..=10);
        if mascota.is_dead() {
            println!("Your pet is dead :(");
            break;
        }
        if mascota.hunger % 2u8 == 0 {
            msg = String::from("..");
        } else {
            msg = String::from("meaw");
        }
		println!("Hunger: {}", mascota.hunger);
        print!("{}", frame);
        println!("\n{} ..", msg);
        thread::sleep(time::Duration::from_millis(MILLIS));
        mascota.tick(number);
        clear_terminal();
    }
}

fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}

struct Pet {
    hunger: u8,
    boredom: u8,
    tiredness: u8,
    dirtiness: u8,
    happy: bool,
}
impl Pet {
    fn new() -> Pet {
        Pet {
            hunger: 0,
            boredom: 0,
            tiredness: 0,
            dirtiness: 0,
            happy: true,
        }
    }
    fn feed(&mut self) {
        self.hunger -= 1;
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
    fn tick(&mut self, number: u8) {
		if number % 2 == 0 {
			self.hunger += 1;
			if number > 5 {
				self.tiredness += 1;
			}
		}
		if number % 3 == 0 {
			self.boredom += 1;
		}
        self.dirtiness += 1;
    }
    fn is_alive(&self) -> bool {
        self.hunger < 100 && self.boredom < 100 && self.tiredness < 100 && self.dirtiness < 100
    }
    fn is_happy(&mut self) -> bool {
        self.happy =
            self.hunger < 50 && self.boredom < 50 && self.tiredness < 50 && self.dirtiness < 50;
        self.happy
    }
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
