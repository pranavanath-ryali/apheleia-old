#[derive(Clone)]
pub struct Cell {
    c: char
}

struct Buffer {
    width: usize,
    height: usize,
    cells: Vec<Vec<Cell>>
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        let default_cell = Cell {
            c: ' '
        };

        Self {
            width,
            height,
            cells: vec![vec![default_cell; width]; height]
        }
    }
    
    fn set(mut self, x: usize, y: usize, c: char) {
        if x >= self.width || y >= self.height {
            return;
        }

        self.cells[y][x] = Cell { c };
    }
}
