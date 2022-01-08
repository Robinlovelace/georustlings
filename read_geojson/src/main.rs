use geojson::GeoJson;
use serde_json::from_str;

fn main() {
    let geojson_path = "../data/geojson/countries.geojson";
    let geojson_input = std::fs::read_to_string(geojson_path).unwrap();
    let geojson: GeoJson = serde_json::from_str(&geojson_input);
    let features = geojson.features();
    println!("{:?}", geojson.into_iter().count());
    // let features = geojson.features();
    let slice = &geojson_input[..500];
    println!("{}", slice);
}
