use std::fmt::{Debug, Formatter, Result};

pub struct PetPiece {
    pub character: char,
    pub coords: (usize, usize), // (row, column)
}

// TODO: Maybe rename this to PetCanvas? since this will handle both the "drawing" and "animation"
// of the pets "canvas" seems more fitting
pub struct PetGrid {
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
    // template: Vec<Vec<char>>, // Template,
    // eyes: Vec<PetPiece>,
    // mouth: PetPiece,
    // tails: Vec<PetPiece>,
    pub vect: Vec<Vec<char>>,
}

impl PetGrid {
    // TODO: For pet templates, meaning, the outline / sillouete of pets, ideally we want to be
    // able to define them in json files or something similar and then have a way to import the and
    // then pass that as the base for the PetGrid.
    // Said templates should also somehow define constraints of where certain body parts can go, so that we don't end up
    // with like floating parts, something like a list that tells us "here are the possible
    // positions for a part (be it eye, mouth, tail, nose, whateva)" if the constraint is shared for
    // all parts then i could see it being posible to, say, place an eye where the mouth would normaly be, or
    // viceversa, we're thinking big customizability here but without making it a pain in the ass
    // to code/configure (us, players) y'know.
    pub fn new(
        template: Vec<Vec<char>>,
        eyes: Vec<PetPiece>,
        mouth: PetPiece,
        tails: Vec<PetPiece>,
    ) -> PetGrid {
        println!("[Pet Grid] Arranging pet parts...");

        // Draw/place the outline of the pet
        let mut pet_vect = template.clone();

        // Draw/place the eye/s in the grid
        for eye in &eyes {
            pet_vect[eye.coords.0][eye.coords.1] = eye.character;
        }

        // Draw/place the mouth in the grid
        pet_vect[mouth.coords.0][mouth.coords.1] = mouth.character;

        // Draw/place the tail/s in the grid
        for tail in &tails {
            pet_vect[tail.coords.0][tail.coords.1] = tail.character;
        }

        println!("[Pet Grid] Pet parts successfully arranged!");

        PetGrid {
            // template,
            // eyes,
            // mouth,
            // tails,
            vect: pet_vect,
        }
    }
}

impl Debug for PetGrid {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for row in self.vect.as_slice() {
            for character in row {
                write!(f, "{:1}", character)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
