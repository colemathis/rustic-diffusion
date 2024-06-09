mod grid;
mod solver;
mod visualization;

fn main() {
    let mut grid = grid::Grid::new(5, 5, 0.0); // Create a small 5x5 grid with initial concentration of 0.0
    grid.set_value(2, 2, 1.0); // Set a central point with higher concentration

   println!("Initial grid:");
   grid.print();

    // Parameters for the diffusion
    let dt = 0.1; // Time step
    let dx = 1.0; // Spatial step in the x direction
    let dy = 1.0; // Spatial step in the y direction
    let D = 0.1; // Diffusion coefficient

   // save
   visualization::visualize_grid(&grid, "diffusion_output_1.png").expect("Failed to visualize grid");
   // Perform a single diffusion step
   solver::diffuse(&mut grid, dt, dx, dy, D);
   // save
   visualization::visualize_grid(&grid, "diffusion_output_2.png").expect("Failed to visualize grid");
   
    println!("Grid after one diffusion step:");
    grid.print();
}
