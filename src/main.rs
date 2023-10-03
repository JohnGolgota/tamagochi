pub mod pet;
pub mod menu;
use pet::Pet;
use std::{thread, time, sync::mpsc};
use rand::Rng;

use crate::menu::Menu;
use crate::pet::PetStatus;

fn main() {
	let mut rng = rand::thread_rng();
    let mut mascota = Pet::new(String::from("perro"));
	let menu: Menu = Menu::new();
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
	let kato_muerto = r#"
  /\_/\
 / x x \
 \¨ ^ ¨/
  /   \    _
 /|_|_|\__/
	"#;
    const MILLIS: u64 = 600;
    let mut msg:&str = "meaw";
	let (tx, rx) = mpsc::channel();

	let input_handle = thread::spawn(move || {
		let mut input = String::new();
		let result = std::io::stdin().read_line(&mut input);
		if result.is_ok() {
			tx.send(input).unwrap();
		}
	});

    for frame in frames.iter().cycle() {
		// clear_terminal();
		let number:u8 = rng.gen_range(0..=10);
		println!("Salud: {}, Hambre: {}, Higiene: {}", mascota.stats.health, mascota.stats.hunger, mascota.stats.dirtiness);
        if mascota.status == PetStatus::Death {
			print!("{}\nRIP {} :(", kato_muerto, mascota.name);
            break;
        }
		print!("{}", menu.options);
		print!("{}", frame);
		println!("\n{} ..", msg);

		let select = rx.try_recv();
		print!("\nselect: {:?}\n", select);

        if number % 2u8 == 0 {
            msg = "..";
        } else {
            msg = "meaw";
        }
        thread::sleep(time::Duration::from_millis(MILLIS));
        mascota.live();
    }
}

fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}


