use geo::Point;

/// Describe the point in an unusual order: "y=..., x=..."
pub fn stringify_point(pt: Point<f64>) -> String {
    // Change me
    // Hint: Use your IDE to figure out what methods exist for Points, or browse through
    // https://docs.rs/geo
    let x = pt.x();
    let y = pt.y();
    format!("y={}, x={}", y, x)
}

#[cfg(test)]
mod tests {
    use super::*;
    use geo::point;

    #[test]
    fn test() {
        let pt = point!(x: 1.23, y: 4.56);
        assert_eq!(stringify_point(pt), "y=4.56, x=1.23");
    }
}
