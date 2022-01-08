use csv;
use std::fs::File;
use geojson::{Feature, FeatureCollection, Value};

fn main() {
    print!("Converting cities.csv to cities.geojson");
    csv_to_geojson();
}

pub fn csv_to_geojson() {
    let mut reader = csv::Reader::from_path("cities.csv").unwrap();
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
    let f = File::create("cities.geojson").unwrap();
    serde_json::to_writer_pretty(f, &fc).unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
