pub mod menu;
pub mod pet;
use pet::Pet;
use rand::Rng;
use std::io::{self, Write};
use std::{sync::mpsc, thread, time};

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
    let mut msg: &str = "meaw";
    let (tx, rx) = mpsc::channel();
    let input_handle = thread::spawn(move || {
        loop {
            let mut input = String::new();
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            tx.send(input.trim().to_string()).unwrap();
            if input.trim() == "exit" {
                break;
            }
        }
    });
    for frame in frames.iter().cycle() {
        clear_terminal();
        let number: u8 = rng.gen_range(0..=10);
        println!(
            "Salud: {}, Hambre: {}, Higiene: {}",
            mascota.stats.health, mascota.stats.hunger, mascota.stats.dirtiness
        );
        if mascota.status == PetStatus::Death {
            print!("{}\nRIP {} :(", kato_muerto, mascota.name);
            break;
        }
        print!("{}", menu.options);
        print!("{}", frame);
        let select = rx.recv().unwrap();
        match select.as_str() {
            "1" => mascota.feed(),
            "2" => mascota.play(),
            "3" => mascota.sleep(),
            "4" => mascota.wash(),
            "5" => break,
            _ =>  println!("\n{} ..", msg),
        }
        if number % 2u8 == 0 {
            msg = "..";
        } else {
            msg = "meaw";
        }
        thread::sleep(time::Duration::from_millis(MILLIS));
        mascota.live();
    }
	input_handle.join().unwrap();
}

fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}
