pub struct Grid{
    width:usize,
    height:usize
}

impl Grid {
    pub fn new(width:usize, height:usize) -> Vec<Vec<&'static str>>{
        Grid{
            width,
            height
        };
        let grid:Vec<Vec<&str>> = vec![vec!["."; width]; height];
        return grid
    }
}