use geo::Geometry;

#[derive(Clone, Debug)]
pub struct State {
    name: String,
    geometry: Geometry,
}

impl State {
    /// Create a new US State with a name and shape
    pub fn new(name: String, geometry: Geometry) -> Self {
        Self { name, geometry }
    }

    /// Get the name of the US State
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the geometry of the US State
    pub fn geometry(&self) -> &Geometry {
        &self.geometry
    }
}
