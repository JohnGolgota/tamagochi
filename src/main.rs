pub mod menu;
pub mod pet;
mod utils;
use rand::Rng;
use std::{thread, time};

use crate::menu::Menu;
use pet::Pet;
use pet::CatGrid;
use pet::PetStatus;

fn main() {
    let mut rng = rand::thread_rng();
    let mascota = &mut Pet::new(String::from("perro"));

    let new_grid = CatGrid::new(pet::CatSize::default()); //(12,6);

    let menu: Menu = Menu::new();
    let kato_muerto = r#"
 /\_/\
/ x x \
\¨ ^ ¨/
 /   \    _
/|_|_|\__/
	"#;
    let _rip_message = r#"
 /\_/\
/ x x \
\¨ ^ ¨/
 /   \    _
/|_|_|\__/
	"#;

    print!("dimensions: {:?} \n", &new_grid.dimensions());
    const MILLIS: u64 = 1000;
    let mut msg: &str = "";
    // let (tx, rx) = mpsc::channel();
    // let input_handle = thread::spawn(move || {
    //     loop {
    //         let mut input = String::new();
    //         //io::stdout().flush().unwrap();
    //         io::stdin().read_line(&mut input).unwrap();
    //         tx.send(1).unwrap();
    //         print!("wha-ta-fak");
    //         if input.trim() == "exit" {
    //             print!("nos fuimos");
    //             //break;
    //         }
    //     }
    // });
    //println!("1:{}", &new_grid.frames[0]);
    //println!("2:{}", &new_grid.frames[1]);
    //println!("3:{}", &new_grid.frames[2]);
    //println!("4:{}", &new_grid.frames[3]);
    //println!("5:{}", &new_grid.frames[4]);

    for frame in new_grid.frames.iter().cycle() {
        clear_terminal();
        if mascota.status == PetStatus::Death {
            print!("{}\nRIP {} :(", kato_muerto, mascota.name);
            break;
        }
        let number: u8 = rng.gen_range(0..=10);
        println!(
            "Salud: {}, Hambre: {}, Higiene: {}, Aburrimiento: {}, Cansancio: {}, Suciedad: {}",
            mascota.stats.health, mascota.stats.hunger, mascota.stats.dirtiness, mascota.stats.boredom, mascota.stats.tiredness, mascota.stats.dirtiness
        );
        print!("{}\n", menu.options);
        print!("printing frame...");
        print!("\n{}", frame);
        println!("{}", msg);
        // let select = rx.recv().unwrap();
        // match select.as_str() {
        //     "1" => mascota.feed(),
        //     "2" => mascota.play(),
        //     "3" => mascota.sleep(),
        //     "4" => mascota.wash(),
        //     "5" => break,
        //     _ =>  println!("{}", msg),
        // };
        if number % 2u8 == 0 {
            msg = "..";
        } else {
            msg = "meaw";
        }
        mascota.live();
        thread::sleep(time::Duration::from_millis(MILLIS));
    }
	//input_handle.join().unwrap();
}

fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}

fn render_frames(grid:&Vec<Vec<char>>) -> String{
    let mut frame = String::new();
    for row in grid {
        let mut rowstr: String = String::default();
        for char  in row {
            rowstr.push(*char);

        }
        frame.push_str(&(rowstr.clone() + "\n"));
        //print!("\n{}", rowstr);
    }
    return frame.clone();
}

