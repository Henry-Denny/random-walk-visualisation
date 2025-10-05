use plotters::prelude::*;

use random_walk::{Point, RandomWalk};

pub mod random_walk;

fn main() {
    let drawing_area = BitMapBackend::gif("random-walk.gif", (900, 900), 250)
        .expect("Should be able to create new gif")
        .into_drawing_area()
        .margin(30, 30, 30, 30);

    drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&drawing_area)
        .caption("Random Walk", ("sans-serif", 40).into_font())
        // Set the size of the label region
        .build_cartesian_2d(-10..10, -10..10)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    // Draw origin
    chart
        .draw_series(PointSeries::of_element([(0, 0)], 5, &BLACK, &|c, s, st| {
            return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
            + Circle::new((0,0),s,st.filled()) // At this point, the new pixel coordinate is established
            + Text::new("O", (-25, 5), ("sans-serif", 30).into_font());
        }))
        .expect("Should be able to draw origin");

    let rw: Vec<Point> = RandomWalk::new(25).collect();
    rw.windows(2).for_each(|line| {
        chart
            .draw_series(LineSeries::new(
                line.iter().copied(),
                Into::<ShapeStyle>::into(RED).stroke_width(2).filled(),
            ))
            .expect("Should be able to plot random walk");
        drawing_area
            .present()
            .expect("Should be able to render frame");
    });
}
