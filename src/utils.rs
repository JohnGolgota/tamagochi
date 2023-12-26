use std::{fmt::{Debug, self}, path::Iter};

pub struct Grid{
    cols:usize,
    rows:usize,
    pub vect:Vec<Vec<String>>
}

impl Grid {
    pub fn new(cols:usize, rows:usize) -> Grid{
        Grid{
            cols,
            rows,
            vect: vec![
                vec![
                    ".".to_string(); cols // Vector that contains n amount of String characters, n is determined by the cols parameter
                ]; rows // Vector that contains n amount of Vec<String>, n is determined by the rows parameter
            ]
        }
    }

    /// Returns grid's (aka two dimentional array) dimentions as a tuple, 
    /// where the 1st element is the number of "columns" 
    /// and the 2nd element is the number of "rows"
    /// 
    /// ## Examples
    ///
    /// ```rust
    /// let myGrid = Grid::new(12,5);
    /// assert_eq!(myGrid.dimentions(), (12, 5));
    /// 
    /// ```
    /// # Note
    ///
    /// If for some reason you decide to use this instead of retrieving the structs attributes directly
    /// here's a little comparison between the two methods
    /// 
    /// ```rust
    /// let myGrid = Grid::new(12,5);
    /// 
    /// // Retrieving the attributes
    /// let cols = myGrid.cols; // -> 12
    /// let rows = myGrid.rows; // -> 5
    /// 
    /// // Using this method
    /// dimentions = myGrid.dimentions();
    /// cols = dimentions.1 // -> 12
    /// rows = dimentions.2 // -> 5
    /// ```
    /// 
    /// ### Extra Note
    /// Man this doc comments are a really cool feature aren't they? i spend more time writing these than
    /// actually coding lmao -kolo
    /// 
    pub fn dimentions(&self) -> (usize, usize){
        (self.cols, self.rows)
    }
    // pub fn render(&mut self) -> Grid{
    //     let grid:Vec<Vec<String>> = vec![
    //             vec![
    //                 String::new(); self.width // Vector that contains n amount of String characters, n is determined by the width parameter
    //             ]; self.height // Vector that contains n amount of Vec<String>, n is determined by the height parameter
    //         ];
    //     return grid
    // }
}


// impl Grid {
//     pub fn new(width:usize, height:usize) -> Vec<Vec<&'static str>>{
//         Grid{
//             width,
//             height
//         };
//         let grid:Vec<Vec<&str>> = vec![vec!["."; width]; height];
//         return grid
//     }
// }



// pub trait Thingy {
//     type Grid;
//     const grid:Grid = vec![vec!["."; width]; height];

//     fn new(width:usize, height:usize) -> Grid{
//         Grid{
//             width,
//             height
//         }
//     }
// }

// impl Thingy for Grid{
//     type Grid = Vec<Vec<String>>;

//     fn new(width:usize, height:usize) -> Grid{
//         Grid{
//             width,
//             height
//         };
//         return grid;
//     }
// }

// impl Iterator for Grid{
//     type Item = Vec<String>;

//     fn next(&mut self) -> Option<Self::Item> {
//         todo!()
//     }
// }

impl Debug for Grid{
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

impl Iterator for Grid {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}