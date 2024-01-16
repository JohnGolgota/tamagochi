use std::fmt::{Debug, self};
use crate::pet::CatSize;

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
    pub fn new(cols:usize, rows:usize) -> Grid{
        Grid{
            cols,
            rows,
            vect: vec![
                vec![char::default(); cols ]; rows   // Vector that contains n amount of single characters (char), n is determined by the cols parameter
            ]   // Vector that contains n amount of Vec<char>, n is determined by the rows parameter
        }
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
    /// 
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


#[derive(Clone)]
pub struct CatGrid{
    cols:usize,
    rows:usize,
    pub vect:Vec<Vec<char>>
}

impl CatGrid {
    pub fn new(cat_size:CatSize) -> Grid{
        match cat_size {
            CatSize::Small{width, height} => Grid{
                    cols:width,
                    rows:height,
                    vect: vec![
                        vec![' ','/','\\','_','/','\\','.','.','.','.','.','.'],
                        vec!['/',' ','o',' ','o',' ','\\','.','.','.','.','.'],
                        vec!['\\','¨',' ','^',' ','¨','/','.','.','.','.','.'],
                        vec![' ','/',' ',' ',' ','\\',' ',' ',' ','\\','.','.'],
                        vec!['(','|','_','|','_','|',')','_','_','/','.','.'],
                        vec!['.','.','.','.','.','.','.','.','.','.','.','.']
    
                    ]
                },
            CatSize::Medium{width, height} => Grid{
                cols:width,
                rows:height,
                vect: vec![
                    vec![' ','/', '\\','_','/','\\','.','.','.','.','.','.'],
                    vec!['/',' ','o',' ','o',' ','\\','.','.','.','.','.'],
                    vec!['\\','¨',' ','^',' ','¨','/','.','.','.','.','.'],
                    vec![' ','/',' ',' ',' ','\\',' ',' ',' ','\\','.','.'],
                    vec!['(','|','_','|','_','|',')','_','_','/','.','.'],
                    vec!['.','.','.','.','.','.','.','.','.','.','.','.']

                ]
            },
            CatSize::Big{width, height} => Grid{
                cols:width,
                rows:height,
                vect: vec![
                    vec![' ','/', '\\','_','/','\\','.','.','.','.','.','.'],
                    vec!['/',' ','o',' ','o',' ','\\','.','.','.','.','.'],
                    vec!['\\','¨',' ','^',' ','¨','/','.','.','.','.','.'],
                    vec![' ','/',' ',' ',' ','\\',' ',' ',' ','\\','.','.'],
                    vec!['(','|','_','|','_','|',')','_','_','/','.','.'],
                    vec!['.','.','.','.','.','.','.','.','.','.','.','.']
                ]
            },
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
