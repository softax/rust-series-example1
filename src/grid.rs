//! Conway's Game of Life
//!
//! Example:
//!
//! ```
//! let g = Grid::new(GRID_SIZE);
//! ```

extern crate rand;
use rand::prelude::*;
use rayon::prelude::*;

/// Base Cell structure
#[derive(Clone, Copy)]
pub enum Cell {
    Empty = 0,
    Alive = 1,
}

pub struct Grid {
    pub size: usize,
    pub grid: Vec<Cell>,
    ns_directions: [i32; 9],
}

impl Grid {
    pub fn new(size: usize, init_treshold: f64) -> Grid {
        let mut g = Grid::new_empty(size);
        g.init_random(init_treshold);
        g
    }

    pub fn new_empty(size: usize) -> Grid {
        let size_i = size as i32;
        Grid {
            size,
            grid: vec![Cell::Empty; size * size],
            ns_directions: [
                -size_i - 1,
                -size_i,
                -size_i + 1,
                -1,
                0,
                1,
                size_i - 1,
                size_i,
                size_i + 1,
            ],
        }
    }

    pub fn update(&mut self) {
        let mut new_grid: Grid = Grid::new_empty(self.size);

        new_grid
            .grid
            .par_iter_mut()
            .enumerate()
            .for_each(|(ndx, c)| {
                let ns_count: i32 = self.count_ns(ndx);
                let cell = self.grid[ndx] as i32;

                *c = match ns_count {
                    2 | 3 if cell > 0 => Cell::Alive,
                    3 if cell == 0 => Cell::Alive,
                    _ => Cell::Empty,
                };
            });

        self.grid = new_grid.grid;
    }
}

impl Grid {
    fn init_random(&mut self, threshold: f64) {
        let mut rng = rand::thread_rng();
        for ndx in 0..self.size * self.size {
            let v: f64 = rng.gen();
            if v < threshold {
                self.grid[ndx] = Cell::Alive;
            }
        }
    }

    fn count_ns(&self, ndx: usize) -> i32 {
        let size_i: i32 = self.size as i32;
        let ndx_i: i32 = ndx as i32;

        self.ns_directions
            .iter()
            .map(|offset| offset + (ndx as i32))
            .filter(|&x| {
                let is_in_bounds = x > 0 && x < size_i * size_i;
                let is_not_in_center = x != ndx_i;
                let is_not_wrapped = (x / size_i - ndx_i / size_i).abs() <= 1;

                is_in_bounds && is_not_in_center && is_not_wrapped
            })
            .map(|n| self.grid[n as usize] as i32)
            .sum()
    }

    #[allow(dead_code)]
    fn update_single_threaded(&mut self) {
        let mut new_grid: Grid = Grid::new_empty(self.size);
        new_grid.grid.iter_mut().enumerate().for_each(|(ndx, c)| {
            let ns_count: i32 = self.count_ns(ndx);
            let cell = self.grid[ndx] as i32;

            *c = match ns_count {
                2 | 3 if cell > 0 => Cell::Alive,
                3 if cell == 0 => Cell::Alive,
                _ => Cell::Empty,
            };
        });

        self.grid = new_grid.grid;
    }
}
