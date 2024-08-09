// src/solver.rs
use rand::prelude::*;
use crate::grid::Grid;
/// General continuos diffusion model
pub fn diffuse(grid: &mut super::grid::Grid, dt: f64, dx: f64, dy: f64, D: f64) {

    let rows = grid.rows;
    let cols = grid.cols;
    let mut new_values = vec![vec![0.0; cols]; rows]; // Create a 2D vector for new values

    for i in 0..grid.rows {
        for j in 0..grid.cols {
            let current = grid.get_value(i, j);
            let right = grid.get_value(i.wrapping_add(1), j);
            let left = grid.get_value(i.wrapping_sub(1), j);
            let up = grid.get_value(i, j.wrapping_add(1));
            let down = grid.get_value(i, j.wrapping_sub(1));

            let diff_x = (right - 2.0*current + left) / (dx*dx);
            let diff_y = (up - 2.0*current + down) / (dy*dy);
            let updated = current + dt * D * (diff_x + diff_y);
            new_values[i][j] = updated;
        }
    }
    // Update the grid with new values
    for i in 0..rows {
        for j in 0..cols {
            grid.set_value(i, j, new_values[i][j]);
        }
    }

}


pub fn random_walk(grid: &mut super::grid::Grid, dt: f64, dx: f64, dy: f64, D: f64) {

    let rows = grid.rows;
    let cols = grid.cols;
    let mut new_values = Grid::new(rows, cols, 0.0); // Create a 2D vector for new values

    let mut rng = thread_rng();
    for i in 0..grid.rows {
        for j in 0..grid.cols {
            let current = grid.get_value(i, j);
            let diff_prob = D*current*dt;
            let p: f64 = rng.gen();
            if p <= diff_prob {
                new_values.change_value(i,j, -1.0);
                let dir = rng.gen_range(1..=4);
                
                match dir {
                    1 => new_values.change_value(i.wrapping_add(1),j, 1.0), // right
                    2 => new_values.change_value(i.wrapping_sub(1),j, 1.0),// left
                    3 => new_values.change_value(i,j.wrapping_add(1), 1.0), // up
                    4 => new_values.change_value(i,j.wrapping_sub(1), 1.0), // down
                    _ => println!("error"),
                }
            } 

        }
    }
    // Update the grid with new values
    for i in 0..rows {
        for j in 0..cols {
            grid.change_value(i, j, new_values.get_value(i,j));
        }
    }

}