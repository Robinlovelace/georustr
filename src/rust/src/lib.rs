use std::fs::File;

use csv::Reader;
use extendr_api::prelude::*;
use geo::{winding_order::Points, Coordinate, Point};
use geojson::{Feature, GeoJson, Geometry, Value};
use serde_json::to_string_pretty;

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
    "Hello world!"
}
/// Convert `points.csv` to points_rust.geojson.
/// @export
#[extendr]
pub fn csv_to_geojson() {
    let mut reader = csv::Reader::from_path("points.csv").unwrap();
    let points = reader
        .records()
        // this will silently discard invalid / unparseable records
        .filter_map(|record| record.ok())
        .map(|record| {
            Feature::from(Value::from(Point::new(
                record[1].parse::<f64>().unwrap(),
                record[0].parse::<f64>().unwrap(),
            )))
        })
        .collect();
    let fc: FeatureCollection<_> = FeatureCollection {
        bbox: None,
        features: points,
        foreign_members: None,
    };
    let geojson_string = to_string_pretty(&fc).unwrap();
    serde_json::to_writer_pretty(
        &mut File::create("points_rust.geojson").unwrap(),
        &geojson_string,
    )
    .unwrap();
}
// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod georustr;
    fn hello_world;
    fn csv_to_geojson;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(hello_world(), "Hello world!");
    }
    #[test]
    fn csv_to_geojson_works() {
        csv_to_geojson();
    }
}
