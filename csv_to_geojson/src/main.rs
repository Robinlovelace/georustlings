use csv;
use std::fs::File;
use std::path::Path;
use geojson::{Feature, FeatureCollection, Value};

fn main() {
    let csv_file = "cities.csv";
    let geojson_file = "cities.geojson";
    print!("Converting {} to {}", csv_file, geojson_file);
    csv_to_geojson(
        "cities.csv",
        "cities_test.geojson",
    );
}

pub fn csv_to_geojson<T: AsRef<Path>>(csv_file: T, geojson_file: T) {
    let mut points = Vec::new();
    let mut reader = csv::Reader::from_path(csv_file).expect("Couldn't create CSV reader");
    let mut record = csv::ByteRecord::new();
    while reader.read_byte_record(&mut record).expect("Couldn't read CSV record") {
        points.push(Feature::from(Value::Point(vec![
            from_utf8(record.get(0).expect("couldn't retrieve x field from CSV"))
                .expect("Invalid UTF-8 when parsing x coordinate")
                .parse::<f64>()
                .expect("Couldn't convert x coordinate to f64"),
            from_utf8(record.get(1).expect("couldn't retrieve y field from CSV"))
                .expect("Invalid UTF-8 when parsing y coordinate")
                .parse::<f64>()
                .expect("Couldn't convert y coordinate to f64"),
        ])));
    }
    let fc = FeatureCollection {
        bbox: None,
        features: points,
        foreign_members: None,
    };
    let f = File::create(geojson_file).expect("Couldn't create output file");
    let f = BufWriter::new(f);
    serde_json::to_writer_pretty(f, &fc).expect("Couldn't write output GeoJSON to file");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


