use crate::utils::randnum;

#[derive(PartialEq)]
pub enum PartType {
    Eye,
    Mouth,
    Tail,
}

pub struct PetPart {
    pub part_type: PartType,
    pub character: char,
    pub coords: (usize, usize), // (row, column)
}

pub trait CustomDisplay {
    fn print(&self);
}

pub struct PetCanvas {
    /// Creates a new "pet grid"—a 2-dimensional vector with characters at specific positions
    /// according to the provided parameters, that make it so that the printed vector
    /// resembles a pet (hopefully).
    ///
    /// This class is meant to serve as the mechanism that arranges all of the parts of a pet in a
    /// grid and ultimately creates the "frames" that would later be used for the pet animation.
    ///
    /// ### Usage
    /// ```rust
    /// eyes_coords = vec![(1, 2), (1, 4)]
    /// mouth_coords = (1, 2)
    /// tail_coords = vec![(1, 2)]
    /// let myGrid = PetGrid::new(eyes_coords, mouth_coords, tail_coords);
    /// assert_eq!(myGrid, vec![[char::default()12]; 5];);
    /// ```
    parts: Vec<PetPart>,
    pub initial_frame: Vec<Vec<char>>,
    pub death_frame: Vec<Vec<char>>,
}

impl PetCanvas {
    // TODO: For pet templates, meaning, the outline / sillouete of pets, ideally we want to be
    // able to define them in json files or something similar and then have a way to import the and
    // then pass that as the base for the PetGrid.
    // Said templates should also somehow define constraints of where certain body parts can go, so that we don't end up
    // with like floating parts, something like a list that tells us "here are the possible
    // positions for a part (be it eye, mouth, tail, nose, whateva)" if the constraint is shared for
    // all parts then i could see it being posible to, say, place an eye where the mouth would normaly be, or
    // viceversa, we're thinking big customizability here but without making it a pain in the ass
    // to code/configure (us, players) y'know.
    pub fn new(template: Vec<&str>, parts: Vec<PetPart>) -> PetCanvas {
        println!("[Pet Grid] Arranging pet parts...");

        // Draw/place the outline of the pet
        let mut initial_frame: Vec<Vec<char>> =
            template.iter().map(|s| s.chars().collect()).collect();
        let mut death_frame = initial_frame.clone();

        // Draw/place the parts of the pet (eye, mouth, tail, etc)
        for part in &parts {
            initial_frame[part.coords.0][part.coords.1] = part.character;

            // Generate frame for when the pet perishes based on the initial frame
            if part.part_type == PartType::Eye {
                death_frame[part.coords.0][part.coords.1] = 'X';
            } else {
                death_frame[part.coords.0][part.coords.1] = part.character;
            }
        }

        println!("[Pet Grid] Pet parts successfully arranged!");

        PetCanvas {
            parts,
            initial_frame,
            death_frame,
        }
    }

    pub fn generate_frames(&self) -> Vec<Vec<Vec<char>>> {
        let eye_state_2 = '=';
        let mouth_state_2 = 'w';
        let tail_state_2 = '_';

        let nframes = 5;
        let mut frames: Vec<Vec<Vec<char>>> = Vec::new();

        let mut current_frame = self.initial_frame.clone();
        for _frame in 0..nframes {
            frames.push(current_frame.clone());

            // current_frame.print(); // use this for debugging frames
            for part in &self.parts {
                // Draw/place the parts of the pet for the next frame
                match part.part_type {
                    PartType::Eye => {
                        if randnum() % 2 == 0 {
                            current_frame[part.coords.0][part.coords.1] = eye_state_2;
                        } else {
                            current_frame[part.coords.0][part.coords.1] = part.character;
                        }
                    }
                    PartType::Mouth => {
                        if randnum() % 2 == 0 {
                            current_frame[part.coords.0][part.coords.1] = mouth_state_2;
                        } else {
                            current_frame[part.coords.0][part.coords.1] = part.character;
                        }
                    }
                    PartType::Tail => {
                        if randnum() % 2 == 0 {
                            current_frame[part.coords.0][part.coords.1] = tail_state_2;
                        } else {
                            current_frame[part.coords.0][part.coords.1] = part.character;
                        }
                    }
                }
            }
        }
        return frames;
    }
}

impl CustomDisplay for Vec<Vec<char>> {
    fn print(&self) {
        print!("\n");
        for row in self {
            for character in row {
                print!("{}", character);
            }
            print!("\n")
        }
    }
}
