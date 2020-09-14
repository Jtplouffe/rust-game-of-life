use super::Cell;

#[derive(Debug)]
pub struct Board {
    height: u8,
    width: u8,
    pub cells: Vec<Vec<Cell>>,
}

impl Board {
    pub fn new(height: u8, width: u8) -> Self {
        Self {
            height,
            width,
            cells: vec![vec![Cell::new(); height as usize]; width as usize],
        }
    }
}