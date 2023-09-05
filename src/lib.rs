mod utils;

use std::fmt::Display;
use std::time::Duration;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(format!("Hello, this is {name} wasm-game-of-life!").as_str());
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, x) if x == 2 || x == 3 => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, x) if x == 3 => Cell::Alive,
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

}

impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use std::thread::sleep;

    use super::*;

    #[test]
    pub fn test_get_index_work_as_expected() -> () {
        let mut universe = Universe {
            width: 5,
            height: 5,
            cells: vec![Cell::Dead; 25],
        };
        assert_eq!(7, universe.get_index(1, 2));
    }

    #[test]
    pub fn test_neighbours_return_0_with_empty_matrix() -> () {
        let mut universe = Universe {
            width: 5,
            height: 5,
            cells: vec![Cell::Dead; 25],
        };
        for i in 1..6 {
            for j in 1..6 {
                assert_eq!(universe.live_neighbor_count(i, j), 0);
            }
        }
    }

    #[test]
    pub fn test_display_universe_tick() -> () {
        let mut universe = Universe {
            width: 5,
            height: 5,
            cells: vec![
                Cell::Dead,
                Cell::Dead,
                Cell::Dead,
                Cell::Dead,
                Cell::Dead,

                Cell::Dead,
                Cell::Dead,
                Cell::Alive,
                Cell::Dead,
                Cell::Dead,

                Cell::Dead,
                Cell::Dead,
                Cell::Alive,
                Cell::Dead,
                Cell::Dead,

                Cell::Dead,
                Cell::Dead,
                Cell::Alive,
                Cell::Dead,
                Cell::Dead,

                Cell::Dead,
                Cell::Dead,
                Cell::Dead,
                Cell::Dead,
                Cell::Dead,
            ],
        };

        'infinite: loop {
            println!("{universe}");
            universe.tick();
            sleep(Duration::from_millis(500));
        }
        
    }
}
