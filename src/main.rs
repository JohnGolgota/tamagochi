
use std::{thread, time};

fn main() {
	let mut mascota = Pet::new();
    let frame1 = r#"
      /\_/\
     / @ @ \
    ( >    <)
     '>>-<<'
      / o \
    "#;
    let frame2 = r#"
      /\_/\
     / o o \
    ( >    <)
     '>>-<<'
      / o \
    "#;
    let millis: u64 = 500;
	let mut msg = String::from("meaw");

    loop {

		if mascota.is_dead() {
			println!("Your pet is dead :(");
			break;
		}
        print!("{}", frame1);
		println!("\n{} ..", msg);
        msg = String::from("..");
        thread::sleep(time::Duration::from_millis(millis));
        clear_terminal();

        print!("{}", frame2);
		println!("\n{} ..", msg);
        msg = String::from("meaw");
        thread::sleep(time::Duration::from_millis(millis));
        clear_terminal();
		// println!("hunger: {}", mascota.hunger);
		// print pet status

        mascota.tick();
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
	fn tick(&mut self) {
		self.hunger += 1;
		self.boredom += 1;
		self.tiredness += 1;
		self.dirtiness += 1;
	}
	fn is_alive(&self) -> bool {
		self.hunger < 100 && self.boredom < 100 && self.tiredness < 100 && self.dirtiness < 100
	}
	fn is_happy(&mut self) -> bool {
		self.happy = self.hunger < 50 && self.boredom < 50 && self.tiredness < 50 && self.dirtiness < 50;
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
