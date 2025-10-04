use plotters::prelude::*;

pub mod random_walk;

fn main() {
    let drawing_area = BitMapBackend::new("random-walk.png", (900, 900))
        .into_drawing_area()
        .margin(30, 30, 30, 30);

    drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&drawing_area)
        .caption("Random Walk", ("sans-serif", 40).into_font())
        // Set the size of the label region
        .x_label_area_size(20)
        .y_label_area_size(20)
        .build_cartesian_2d(-10..10, -10..10)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    chart
        .draw_series(LineSeries::new(
            vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2), (3, 2)],
            &RED,
        ))
        .unwrap();
}
