use std::fs::File;
use std::io::BufWriter;

use extendr_api::prelude::*;
use geojson::{Feature, FeatureCollection, Value};

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
    let mut reader = csv::Reader::from_path("points.csv").unwrap();
    let points = reader
        .records()
        // this will silently discard invalid / unparseable records
        .filter_map(|record| record.ok())
        .map(|record| {
            Feature::from(Value::Point(vec![
                record[0].parse::<f64>().unwrap(),
                record[1].parse::<f64>().unwrap(),
            ]))
        })
        .collect();
    let fc: FeatureCollection = FeatureCollection {
        bbox: None,
        features: points,
        foreign_members: None,
    };
    let f = File::create("points_rust.geojson").unwrap();
    let f = BufWriter::new(f);
    serde_json::to_writer_pretty(f, &fc).unwrap();
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
    fn csv_to_geojson_works() {
        csv_to_geojson_rust();
    }
}
