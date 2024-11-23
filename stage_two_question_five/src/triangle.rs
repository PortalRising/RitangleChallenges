use glam::DVec3;

use crate::point;

pub struct Triangle {
    point_p: DVec3,
    point_q: DVec3,
    point_r: DVec3,
}

impl Triangle {
    /// Create a new triangle from t
    pub fn new(t: f64) -> Self {
        Self {
            point_p: point::new_p(t),
            point_q: point::new_q(t),
            point_r: point::new_r(t),
        }
    }

    /// Calculate the area of the triangle
    pub fn area(&self) -> f64 {
        // An implementation of Heron's formula
        let side_a = self.point_p.distance(self.point_q);
        let side_b = self.point_p.distance(self.point_r);
        let side_c = self.point_q.distance(self.point_r);

        let perimeter = side_a + side_b + side_c;
        let half_perimeter = perimeter / 2.0;

        let area = (half_perimeter
            * (half_perimeter - side_a)
            * (half_perimeter - side_b)
            * (half_perimeter - side_c))
            .sqrt();

        return area;
    }
}
