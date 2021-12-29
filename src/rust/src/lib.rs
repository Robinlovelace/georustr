use std::fs::File;

use extendr_api::prelude::*;
use csv::Reader;
use geo::{Point, winding_order::Points, Coordinate};
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
    // // Test data:
    // let geometry = Geometry::new(Value::Point(vec![-120.66029, 35.2812]));
    // let geojson = GeoJson::Feature(Feature {
    //     bbox: None,
    //     geometry: Some(geometry),
    //     id: None,
    //     properties: None,
    //     foreign_members: None,
    // });
    // let geojson_string = geojson.to_string();
    // serde_json::to_writer_pretty(&mut File::create("points_rust.geojson").unwrap(), &geojson_string).unwrap();

    // let mut points = Vec::new();
    let mut points_geojson = Vec::new();
    let mut reader = csv::Reader::from_path("points.csv").unwrap();
    
  for result in reader.records() {
        let record = result.unwrap();
        let lat = record[0].parse::<f64>().unwrap();
        let lon = record[1].parse::<f64>().unwrap();
        // points.push(Point::new(lon, lat));
        points_geojson.push(Geometry::new(Value::Point(vec![lon, lat])));

    }
    let geojson = points_geojson.into_iter().collect::<GeoJson>();
    // print!("{}", to_string_pretty(&geojson).unwrap());
    // let geojson_string = to_string_pretty(&geojson).unwrap();
    serde_json::to_writer_pretty(&mut File::create("points_rust.geojson").unwrap(), &geojson).unwrap();
    // let geojson = GeoJson::Feature(Feature {
    //     bbox: None,
    //     geometry: Some(Geometry::new(Value::MultiPoint(points_value))),
    //     id: None,
    //     properties: None,
    //     foreign_members: None,
    // });
    // let gjstring = to_string_pretty(&geojson).unwrap();
    // println!("{}", gjstring);
    // serde_json::to_writer_pretty(&File::create("points_rust.geojson"), &geojson);
    // geojson.to_file("points_rust.geojson").unwrap();
    // geojson_string.to_file("points_rust.geojson");
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