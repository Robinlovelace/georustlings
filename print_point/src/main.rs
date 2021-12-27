use geo::point;
use print_point::stringify_point;

fn main() {
    print!("{}", stringify_point(point!(x: 1.23, y: 4.56)));
}