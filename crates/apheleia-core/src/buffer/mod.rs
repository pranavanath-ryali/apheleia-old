#[derive(Clone)]
struct Cell {
    c: char,
}

pub struct Buffer {
    pub width: usize,
    pub height: usize,
    cells: Vec<Vec<Cell>>,
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        let default_cell = Cell { c: ' ' };

        Self {
            width,
            height,
            cells: vec![vec![default_cell; width]; height],
        }
    }

    pub fn new_fill(width: usize, height: usize, c: char) -> Self {
        let default_cell = Cell { c };

        Self {
            width,
            height,
            cells: vec![vec![default_cell; width]; height],
        }
    }

    pub fn get(&mut self, x: usize, y: usize) -> char {
        self.cells[y][x].c
    }

    pub fn set(&mut self, x: usize, y: usize, c: char) {
        if x >= self.width || y >= self.height {
            return;
        }

        self.cells[y][x] = Cell { c };
    }
}
