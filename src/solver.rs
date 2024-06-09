// src/solver.rs
pub fn diffuse(grid: &mut super::grid::Grid, dt: f64, dx: f64, dy: f64, D: f64) {

    let rows = grid.rows;
    let cols = grid.cols;
    let mut new_values = vec![vec![0.0; cols]; rows]; // Create a 2D vector for new values

    for i in 1..grid.rows-1 {
        for j in 1..grid.cols-1 {
            let current = grid.get_value(i, j);
            let right = grid.get_value(i+1, j);
            let left = grid.get_value(i-1, j);
            let up = grid.get_value(i, j+1);
            let down = grid.get_value(i, j-1);

            let diff_x = (right - 2.0*current + left) / (dx*dx);
            let diff_y = (up - 2.0*current + down) / (dy*dy);
            let updated = current + dt * D * (diff_x + diff_y);
            new_values[i][j] = updated;
        }
    }
    // Update the grid with new values
    for i in 1..rows-1 {
        for j in 1..cols-1 {
            grid.set_value(i, j, new_values[i][j]);
        }
    }

}
