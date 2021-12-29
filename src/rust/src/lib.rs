use std::fs::File;

use csv::Reader;
use extendr_api::prelude::*;
use geo::{winding_order::Points, Coordinate, Point};
use geojson::{FeatureCollection, Feature, Geometry, Value};
use serde_json::to_string_pretty;
use serde::Deserialize;

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
    "Hello world!"
}
/// Convert `points.csv` to points_rust.geojson.
/// @export
#[extendr]
pub fn csv_to_geojson_rust() {
    let geojson_path = "points_rust.geojson";
    let csv_path = "points.csv";
    let mut reader = csv::Reader::from_path(csv_path).unwrap();

    // #[derive(Debug, Deserialize)]
    // struct Rustpoint {
    //   x: f64,
    //   y: f64,
    // }


    let mut points = Vec::new();
    // let mut points_geojson = Vec::new();
    let mut reader = csv::Reader::from_path("points.csv").unwrap();

  for result in reader.records() {
        let record = result.unwrap();
        let lat = record[0].parse::<f64>().unwrap();
        let lon = record[1].parse::<f64>().unwrap();
        points.push(Point::new(lon, lat));
        // points_geojson.push(Geometry::new(Value::Point(vec![lon, lat])));

    }



    // let points = reader
    //     .records()
    //     // this will silently discard invalid / unparseable records
    //     .filter_map(|record| record.ok())
    //     .map(|record| {
    //         Feature::from(Value::from(Value::Point(Rustpoint {
    //             x: record[0],
    //             y: record[1],
    //         })))
    //     })
    //     .collect();

    // let fc: FeatureCollection<> = FeatureCollection {
    //     bbox: None,
    //     features: points,
    //     foreign_members: None,
    // };
    // let geojson_string = to_string_pretty(&fc).unwrap();
    // serde_json::to_writer_pretty(
    //     &mut File::create(geojson_path).unwrap(),
    //     &fc,
    // )
    // .unwrap();
}
// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod georustr;
    fn hello_world;
    fn csv_to_geojson_rust;
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
        csv_to_geojson_rust();
    }
}
