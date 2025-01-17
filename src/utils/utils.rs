use std::fmt::{Debug, self};
use crate::pet::CatSize;
use std::collections::HashMap;

enum GridType{
    RegularGrid,
    CatGrid
}

#[derive(Clone)]
pub struct Grid{
    cols:usize,
    rows:usize,
    pub vect:Vec<Vec<char>>
}

impl Grid {
    /// Creates a new grid (2-dimensional vector) with the given dimentions (cols and rows)
    /// where every character is a blank space ('x00')
    ///
    /// ### Usage
    ///
    /// ```rust
    /// let myGrid = Grid::new(12,5);
    /// assert_eq!(myGrid, vec![[char::default()12]; 5];);
    /// ```
    pub fn new(cols: usize, rows:usize, cat_size:Option<CatSize>, grid_type:Option<GridType>) -> Grid{

        let grid_type = grid_type.unwrap_or(GridType::RegularGrid);
        let cat_size = cat_size.unwrap_or(CatSize::Medium { width: 12, height: 7 });

        match grid_type {
            GridType::RegularGrid => {
                let vect = vec![
                    vec![char::default(); cols ]; rows   // Vector that contains n amount of single characters (char), n is determined by the cols parameter
                ];

                Grid{
                    cols,
                    rows,
                    vect: vec![
                        vec![char::default(); cols ]; rows   // Vector that contains n amount of single characters (char), n is determined by the cols parameter
                    ]   // Vector that contains n amount of Vec<char>, n is determined by the rows parameter
                }
            }
            GridType::CatGrid => {
                match cat_size {
                    // CatSize::Small{width, height} => {
                    //     let vect: Vec<Vec<char>> = vec![
                    //         vec!['.','.','.','.','.','.','.','.','.','.','.','.'],
                    //         vec!['.',' ','/','\\','_','/','\\','.','.','.','.','.'],
                    //         vec!['.','/',' ','o',' ','o',' ','\\','.','.','.','.'],
                    //         vec!['.','\\','¨',' ','^',' ','¨','/','.','.','.','.'],
                    //         vec!['.',' ','/',' ',' ',' ','\\',' ',' ',' ','\\','.'],
                    //         vec!['.','(','|','_','|','_','|',')','_','_','/','.'],
                    //         vec!['.','.','.','.','.','.','.','.','.','.','.','.']
                    //     ];
                    //     let w: usize = width;
                    //     let h: usize = height;

                    //     //let width = &w;
                    //     Grid{
                    //         cols:width,
                    //         rows:height,
                    //         vect
                    //     }

                    // },
                    CatSize::Medium{width, height} => {
                        let vect: Vec<Vec<char>> = vec![
                            vec!['.','.','.','.','.','.','.','.','.','.','.','.'],
                            vec!['.',' ','/','\\','_','/','\\','.','.','.','.','.'],
                            vec!['.','/',' ','o',' ','o',' ','\\','.','.','.','.'],
                            vec!['.','\\','¨',' ','^',' ','¨','/','.','.','.','.'],
                            vec!['.',' ','/',' ',' ',' ','\\',' ',' ',' ','\\','.'],
                            vec!['.','(','|','_','|','_','|',')','_','_','/','.'],
                            vec!['.','.','.','.','.','.','.','.','.','.','.','.']
                        ];
                        let w: usize = width;
                        let h: usize = height;

                        //let width = &w;
                        Grid{
                            cols:width,
                            rows:height,
                            vect
                        }

                    },
                    // CatSize::Big{width, height} => {
                    //     let vect: Vec<Vec<char>> = vec![
                    //         vec!['.','.','.','.','.','.','.','.','.','.','.','.'],
                    //         vec!['.',' ','/','\\','_','/','\\','.','.','.','.','.'],
                    //         vec!['.','/',' ','o',' ','o',' ','\\','.','.','.','.'],
                    //         vec!['.','\\','¨',' ','^',' ','¨','/','.','.','.','.'],
                    //         vec!['.',' ','/',' ',' ',' ','\\',' ',' ',' ','\\','.'],
                    //         vec!['.','(','|','_','|','_','|',')','_','_','/','.'],
                    //         vec!['.','.','.','.','.','.','.','.','.','.','.','.']
                    //     ];
                    //     let w: usize = width;
                    //     let h: usize = height;

                    //     //let width = &w;
                    //     Grid{
                    //         cols:width,
                    //         rows:height,
                    //         vect
                    //     }

                    // },
                }
                // let width = &w;
                //     Grid{
                //         cols:&w,
                //         rows:height,
                //         vect
                //     }
            }
            _ => Grid{
                cols:5,
                rows:5,
                vect: vec![]
            }
        }


        // Vector that contains n amount of Vec<char>, n is determined by the rows parameter

    }

    /// Creates a new grid with the given dimentions (cols and rows)
    /// where every character is the given `character` param
    ///
    /// ### Usage
    ///
    /// ```rust
    /// let myGrid = Grid::new_from('💖',12,5);
    /// assert_eq!(myGrid, vec![vec![character; 12 ]; 6);
    /// ```
    /// printed output would look like this:
    /// ```
    /// 💖💖💖💖💖💖💖💖💖💖💖💖
    /// 💖💖💖💖💖💖💖💖💖💖💖💖
    /// 💖💖💖💖💖💖💖💖💖💖💖💖
    /// 💖💖💖💖💖💖💖💖💖💖💖💖
    /// 💖💖💖💖💖💖💖💖💖💖💖💖
    /// 💖💖💖💖💖💖💖💖💖💖💖💖
    /// ```
    pub fn new_from(character:char, cols:usize, rows:usize) -> Grid{
        Grid{
            cols,
            rows,
            vect: vec![
                vec![character; cols ]; rows   // Vector that contains n amount of single characters (char), n is determined by the cols parameter
            ]   // Vector that contains n amount of Vec<char>, n is determined by the rows parameter
        }
    }

    /// Returns grid dimentions as a tuple,
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
}

pub struct CharInfo {
    pub character:char,
    pub coords:(usize, usize),
}

impl CharInfo {
    pub fn new(character:char, coords:(usize, usize)) -> CharInfo {
        CharInfo{
            character,
            coords
        }
    }
}

pub struct DoubleCharInfo {
    pub character:char,
    pub coords:((usize, usize), (usize, usize)),
}

impl DoubleCharInfo {
    pub fn new(character:char, coords:((usize, usize), (usize, usize))) -> DoubleCharInfo {
        DoubleCharInfo{
            character,
            coords
        }
    }
}

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
