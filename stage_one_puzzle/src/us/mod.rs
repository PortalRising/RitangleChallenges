use std::sync::OnceLock;

use geo::GeometryCollection;
use geojson::{quick_collection, GeoJson};

/// Store the contents of the GeoJSON file containing the US State boundaries here
const STATE_GEOJSON: &'static str = include_str!("cb_2018_us_state_20m.geojson");

pub struct UnitedStatesLookup;

impl UnitedStatesLookup {
    /// Get the json file that stores our state polygons as a struct
    fn state_geo_json() -> &'static GeometryCollection {
        static GEO_DATA: OnceLock<GeometryCollection> = OnceLock::new();

        GEO_DATA.get_or_init(|| {
            let geo_data = STATE_GEOJSON.parse().unwrap();
            quick_collection(&geo_data).unwrap()
        })
    }

    /// Get every state polygon
    fn state_polygons() {}

    /// Get the state that the GPS coordinate is on, as long as it is on one inside the
    /// contiential United States (This means it excludes Hawaii and Alaska from the search)
    pub fn gps_to_state() -> Option<String> {
        None
    }
}
