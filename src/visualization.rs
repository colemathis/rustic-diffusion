// src/visualization.rs
use plotters::prelude::*;

pub fn visualize_grid(grid: &super::grid::Grid, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(file_path, (600, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Diffusion Model", ("sans-serif", 50))
        .build_cartesian_2d(0..grid.cols, 0..grid.rows)?;

    chart.configure_mesh().draw()?;

    // Mapping grid values to colors
    let max_value = grid.get_max();// Find the max value for normalization
    chart.draw_series(
        (0..grid.rows).flat_map(move |y| {
            (0..grid.cols).map(move |x| {
                let value = grid.get_value(y, x);
                let intensity = (value / max_value * 255.0) as u8;
                let color = RGBColor(intensity, intensity, intensity);
                
                Rectangle::new(
                    [(x, y), (x + 1, y + 1)],
                    ShapeStyle::from(&color).filled(),
                )
            })
        })
    )?;

    root.present()?;
    Ok(())
}
