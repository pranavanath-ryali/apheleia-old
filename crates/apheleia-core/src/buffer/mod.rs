#[derive(Clone)]
struct Cell {
    c: char,
}

pub struct Buffer {
    pub width: u16,
    pub height: u16,
    cells: Vec<Vec<Cell>>,
    updated_cells: Vec<(u16, u16, char)>
}

impl Buffer {
    pub fn new(width: u16, height: u16) -> Self {
        let default_cell = Cell { c: ' ' };

        Self {
            width,
            height,
            cells: vec![vec![default_cell; width as usize]; height as usize],
            updated_cells: vec![]
        }
    }

    pub fn new_fill(width: u16, height: u16, c: char) -> Self {
        let default_cell = Cell { c };

        Self {
            width,
            height,
            cells: vec![vec![default_cell; width as usize]; height as usize],
            updated_cells: vec![]
        }
    }

    pub fn get(&mut self, x: u16, y: u16) -> char {
        self.cells[y as usize][x as usize].c
    }

    fn set(&mut self, x: u16, y: u16, c: char) {
        if x >= self.width || y >= self.height {
            return;
        }

        self.cells[y as usize][x as usize] = Cell { c };
        self.updated_cells.push((x, y, c));
    }

    pub fn write_line(&mut self, start_pos_x: u16, start_pos_y: u16, text: &str) {
        for (i, c) in text.chars().enumerate() {
            self.set(start_pos_x + (i as u16), start_pos_y, c);
        }
    }

    pub fn get_update_list(&mut self) -> &Vec<(u16, u16, char)> {
        &self.updated_cells
    }

    pub fn clear_update_list(&mut self) {
        self.updated_cells.clear();
    }
}
