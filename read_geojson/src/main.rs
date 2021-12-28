use geojson::GeoJson;

fn main() {
    let geojson_path = "../data/geojson/countries.geojson";
    let geojson_input = std::fs::read_to_string(geojson_path).unwrap();
    let slice = &geojson_input[..500];
    println!("{}", slice);
    // println!("{:?}", zones);
}
