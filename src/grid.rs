// In src/grid.rs
// src/grid.rs
use ndarray::Array2;

pub struct Grid {
    data: Array2<f64>,  // 2D array of f64 to store the grid values
    pub rows: usize,
    pub cols: usize,
}

impl Grid {
    /// Creates a new grid with specified dimensions and initial value.
    pub fn new(rows: usize, cols: usize, initial_value: f64) -> Grid {
        let data = Array2::<f64>::from_elem((rows, cols), initial_value);
        Grid { data, rows, cols }
    }

    /// Gets the value at a specified (row, col) index.
    pub fn get_value(&self, row: usize, col: usize) -> f64 {
        let r = row % self.rows;
        let c = col % self.cols;
        self.data[[r, c]]
    }
    /// Sets the value at a specified (row, col) index.
    pub fn set_value(&mut self, row: usize, col: usize, value: f64) {
        let r = row % self.rows;
        let c = col % self.cols;
        self.data[[r, c]] = value;
    }
    /// Change the value at a specified (row, col) index.
    pub fn change_value(&mut self, row: usize, col: usize, delta: f64) {
        let r = row % self.rows;
        let c = col % self.cols;
        self.data[[r, c]] += delta;
    }

    /// Print the grid values for debugging purposes.
    pub fn print(&self) {
        println!("{:?}", self.data);
    }
    /// Get the max value
    pub fn get_max(&self) -> f64 {
        self.data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b))
    }

    /// Calculates the sum of all values in the grid
    pub fn sum(&self) -> f64 {
        let mut total = 0.0;
        for i in 0..self.rows {
            for j in 0..self.cols {
                total += self.get_value(i, j);
            }
        }
        total
    }
}
