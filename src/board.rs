use super::Cell;

#[derive(Debug)]
pub struct Board {
    height: u32,
    width: u32,
    pub cells: Vec<Vec<Cell>>,
}

impl Board {
    pub fn new(height: u32, width: u32) -> Self {
        Self {
            height,
            width,
            cells: vec![vec![Cell::new(); height as usize]; width as usize],
        }
    }
}