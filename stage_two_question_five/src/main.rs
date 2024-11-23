use triangle::Triangle;

pub mod point;
pub mod triangle;

fn main() {
    // Iterate from 0..=2pi at a step size of 2pi / 100000
    let max: usize = 100000;

    let mut max_area = 0.0;
    let mut t_when_max = 0.0;
    for i in 0..max {
        let t = 2.0 * core::f64::consts::PI * (i as f64 / max as f64);

        let triangle = Triangle::new(t);

        let area = triangle.area();

        if area > max_area {
            max_area = area;
            t_when_max = t;
        }
    }

    println!("{} {} {}", max_area, t_when_max, max_area * t_when_max);
}
