use csv;
use std::fs::File;
use std::fs;
use geojson::{Feature, FeatureCollection, Value};

fn main() {
    let csv_file = "cities.csv";
    let geojson_file = "cities.geojson";
    print!("Converting {} to {}", csv_file, geojson_file);
    csv_to_geojson(
        "cities.csv",
        "cities.geojson",
    );
}

pub fn csv_to_geojson(csv_file: &str, geojson_file: &str) {
    let mut reader = csv::Reader::from_path(csv_file).unwrap();
    let points = reader
        .records()
        // this will silently discard invalid / unparseable records
        .filter_map(|record| record.ok())
        .map(|record| {
            Feature::from(Value::Point(vec![
                record[1].parse::<f64>().unwrap(),
                record[2].parse::<f64>().unwrap(),
            ]))
        })
        .collect();
    let fc: FeatureCollection = FeatureCollection {
        bbox: None,
        features: points,
        foreign_members: None,
    };
    if geojson_file == "" {
        println!("{}", serde_json::to_string(&fc).unwrap());
    } 
    else {
        let f = File::create(geojson_file).unwrap();
        serde_json::to_writer_pretty(f, &fc).unwrap()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        csv_to_geojson("cities.csv", "cities_test.geojson");
        fs::metadata("cities_test.geojson").is_ok();
        }
}
