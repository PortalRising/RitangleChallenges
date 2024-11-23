use glam::DVec3;

pub fn new_p(t: f64) -> DVec3 {
    DVec3 {
        x: 2.0,
        y: t.cos(),
        z: t.sin(),
    }
}

/// Create a new point Q from a value of t
pub fn new_q(t: f64) -> DVec3 {
    DVec3 {
        x: t.sin(),
        y: 2.0,
        z: t.cos(),
    }
}

/// Create a new point R from a value of t
pub fn new_r(t: f64) -> DVec3 {
    DVec3 {
        x: t.cos(),
        y: t.sin(),
        z: 2.0,
    }
}
