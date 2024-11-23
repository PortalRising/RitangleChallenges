use std::sync::OnceLock;

use geo::{Point, Within};
use geojson::FeatureCollection;
use state::State;

pub mod state;

/// Store the contents of the GeoJSON file containing the US State boundaries here
const STATE_GEOJSON: &'static str = include_str!("cb_2018_us_state_20m.geojson");

pub struct UnitedStatesLookup;

impl UnitedStatesLookup {
    /// Get every state in continental United States as list of its name and geometry
    fn all_continental_states() -> &'static Vec<State> {
        static GEO_DATA: OnceLock<Vec<State>> = OnceLock::new();

        GEO_DATA.get_or_init(|| {
            // Parse US states information from  GeoJSON file
            let geo_data = STATE_GEOJSON.parse().unwrap();

            let FeatureCollection {
                bbox: _,
                features,
                foreign_members: _,
            } = geo_data;

            // Go through each feature, which is a US state and get its geometry and name
            let mut states = Vec::with_capacity(64);
            for feature in features {
                // We know the state name should exist so these unwraps are okay
                let properties = feature.properties.unwrap();
                let mut state_name = properties
                    .get("NAME")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string();

                // Make the state name lowercase as its easier to match and the puzzle needs it later
                state_name.make_ascii_lowercase();

                // We want to exclude alaska and hawaii because they are not part of the continental United States
                // We also exclude Puerto Rico as its not a state
                if ["alaska", "hawaii", "puerto rico"].contains(&&*state_name) {
                    continue;
                }

                // Get state geometry
                let state_geometry = feature.geometry.unwrap();

                states.push(State::new(state_name, state_geometry.try_into().unwrap()));
            }

            // let mut test_output = File::create_new("a.geojson").unwrap();
            // test_output
            //     .write_all(
            //         FeatureCollection::from(&GeometryCollection::from_iter(
            //             states
            //                 .iter()
            //                 .map(|state| state.geometry().clone())
            //                 .collect::<Vec<geo::Geometry>>(),
            //         ))
            //         .to_string()
            //         .as_bytes(),
            //     )
            //     .unwrap();

            states
        })
    }

    /// Check to see if the GPS coordinate is within the continental United State where longitude represents east and latitude represents north
    /// (This means it excludes Hawaii, Alaska, and Puerto Rico from the search)
    pub fn is_within_us<'a>(longitude: f64, latitude: f64) -> bool {
        let test_point = Point::new(longitude, latitude);
        for state in Self::all_continental_states() {
            // Get geometry
            let state_geometry = state.geometry();

            // Check if point is within geometry
            if test_point.is_within(state_geometry) {
                return true;
            }
        }

        // This point is not within a state
        false
    }
}
