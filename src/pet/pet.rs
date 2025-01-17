use std::fmt::{Debug, self};
use std::collections::HashMap;
use rand::prelude::*;

use crate::render_frames;

use super::super::utils::CharInfo;
use super::super::utils::DoubleCharInfo;

#[derive(Debug, PartialEq, Eq)]
pub enum PetStatus {
    Alive,
    Sick,
    Death,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CatSize {
    // Small {
    //     width:usize,
    //     height:usize
    // },
    Medium {
        width:usize,
        height:usize
    },
    // Big {
    //     width:usize,
    //     height:usize
    // }
}

// impl CatSize{
//     pub fn new() -> CatSize{
//         CatSize::Medium {
//             width: 12,
//             height: 6
//         }

//     }
// }

impl Default for CatSize {
    fn default() -> Self { CatSize::Medium { width: 12, height: 6 } }
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

//#[derive(Clone)]
pub struct CatGrid{
    cols:usize,
    rows:usize,
    parts:HashMap<String, CharInfo>,
    double_parts:HashMap<String, DoubleCharInfo>,
    pub vect:Vec<Vec<char>>,
    pub frames: Vec<String>//[&'a str; 5]
}

impl CatGrid{
    pub fn new(cat_size:CatSize) -> CatGrid{
        match cat_size {
            CatSize::Medium{width, height} => {
                let mut cat_grid = CatGrid{
                    cols:width,
                    rows:height,
                    parts:HashMap::from([
                        ("left_cheek".to_string(), CharInfo::new('\\',(2,0))),
                        ("right_cheek".to_string(), CharInfo::new('/',(2,6))),
                        ("mouth".to_string(), CharInfo::new('^',(2,3))),
                        ("tail".to_string(), CharInfo::new('\\',(3,9)))
                    ]),
                    double_parts: HashMap::from([
                        ("ear_left".to_string(), DoubleCharInfo::new('/',((0,1),(0,4)))),
                        ("ear_right".to_string(), DoubleCharInfo::new('\\',((0,2),(0,5)))),
                        ("eyes".to_string(), DoubleCharInfo::new('o',((1,2),(1,4)))),
                        ("blush".to_string(), DoubleCharInfo::new('¨',((2,1),(2,5)))),
                    ]),
                    vect: vec![
                        vec![' ',' ',' ','_',' ',' ','.','.','.','.','.','.'],
                        vec!['/',' ',' ',' ',' ',' ','\\','.','.','.','.','.'],
                        vec![' ',' ',' ','^',' ',' ',' ','.','.','.','.','.'],
                        vec![' ','/',' ',' ',' ','\\',' ',' ',' ','\\','.','.'],
                        vec!['(','|','_','|','_','|',')','_','_','/','.','.'],
                        vec!['.','.','.','.','.','.','.','.','.','.','.','.']
                    ],
                    frames: vec![String::new();5]
                };


                print!("setting single parts...");
                for (part, value) in &cat_grid.parts{
                    let coords = value.coords;
                    let x = coords.1;
                    let y = coords.0;
                    let character = value.character;
                    let name = part;

                    cat_grid.vect[y][x] = character;
                    print!("{name} part set...")
                };

                print!("setting double parts...");
                for (part, value) in &cat_grid.double_parts{
                    let coords1 = value.coords.0;
                    let x1 = coords1.1;
                    let y1 = coords1.0;

                    let coords2 = value.coords.1;
                    let x2 = coords2.1;
                    let y2 = coords2.0;

                    let character = value.character;
                    let name = part;

                    cat_grid.vect[y1][x1] = character;
                    cat_grid.vect[y2][x2] = character;
                    print!("{name} parts set...")
                };

                let mut i = 0;
                while i < cat_grid.frames.len() {
                    let mut grid_frame = cat_grid.vect.clone();
                    if i == 1 || i == 3 {
                        let eyes = &cat_grid.double_parts["eyes"];
                        grid_frame[eyes.coords.0.0][eyes.coords.0.1] = '=';
                        grid_frame[eyes.coords.1.0][eyes.coords.1.1] = '=';
                    }
                    if i == 1 || i == 2 || i == 4 {
                        let tail = &cat_grid.parts["tail"];
                        grid_frame[tail.coords.0][tail.coords.1] = ' ';
                        grid_frame[tail.coords.0][tail.coords.1+1] = '_';
                    }
                    if i == 3 || i == 4 {
                        let mouth = &cat_grid.parts["mouth"];
                        grid_frame[mouth.coords.0][mouth.coords.1] = 'w';
                    }
                    cat_grid.frames[i] = render_frames(&grid_frame);
                    i += 1;
                };

                cat_grid
            }
        }
    }

    /// Returns cat grid dimensions as a tuple,
    /// where the 1st element is the number of "columns"
    /// and the 2nd element is the number of "rows"
    ///
    /// ## Examples
    ///
    /// ```rust
    /// let myCatGrid = CatGrid::new(12,5);
    /// assert_eq!(myCatGrid.dimensions(), (12, 5));
    ///
    /// ```
    /// # Note
    ///
    /// Here's a little comparison between retrieving the struct attributes directly and using this method
    ///
    /// ```rust
    /// let myGrid = Grid::new(12,5);
    ///
    /// // Retrieving the attributes
    /// let cols = myGrid.cols; // -> 12
    /// let rows = myGrid.rows; // -> 5
    ///
    /// // Using this method
    /// dimensions = myGrid.dimensions();
    /// cols = dimensions.1 // -> 12
    /// rows = dimensions.2 // -> 5
    /// ```
    pub fn dimensions(&self) -> (usize, usize){
        (self.cols, self.rows)
    }
}

impl Debug for CatGrid{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.vect.as_slice() {
            for character in row {
                write!(f, "{:1}", character)?;
            }
            writeln!(f)?;
        }
        Ok(())
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
