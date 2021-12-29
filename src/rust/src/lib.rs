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

    #[derive(Deserialize)]
    struct Rustpoint {
      x: f64,
      y: f64,
    }



    for rec in csv::Reader::from_path(csv_path).unwrap().deserialize() {
        let rustpoint: Rustpoint = rec.unwrap();
        print!("{}", rustpoint.x);
        // let point = Point::new(rustpoint.x, rustpoint.y);
        // let feature = Feature::new(
        //     Some(Geometry::new(Value::Point(point))),
        //     None,
        //     None,
        // );
        // let feature_collection = FeatureCollection::new(vec![feature]);
        // let geojson = to_string_pretty(&feature_collection).unwrap();
        // let mut file = File::create(geojson_path).unwrap();
        // file.write_all(geojson.as_bytes()).unwrap();
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
