mod grid;
mod solver;
mod visualization;

fn main() {
    let mut grid = grid::Grid::new(101, 101, 0.0); // Create a small 5x5 grid with initial concentration of 0.0
    grid.set_value(50, 50, 5000.0); // Set a central point with higher concentration

   println!("Initial grid:");
   grid.print();

    // Parameters for the diffusion
    let dt = 0.1; // Time step
    let dx = 0.50; // Spatial step in the x direction
    let dy = 0.50; // Spatial step in the y direction
    let D = 1.0; // Diffusion coefficient

   // save
    visualization::visualize_grid(&grid, "diffusion_output_1.png").expect("Failed to visualize grid");
   // Perform a single diffusion step
   for _i in 1..50000 {
        solver::random_walk(&mut grid, dt, dx, dy, D);
   }
   // save
    visualization::visualize_grid(&grid, "diffusion_output_2.png").expect("Failed to visualize grid");
   
    println!("Grid after  diffusion:");
    grid.print();
    println!("Grid center: {}",grid.get_value(50,50));
    let final_sum = grid.sum();
    println!("{}", final_sum);
}
