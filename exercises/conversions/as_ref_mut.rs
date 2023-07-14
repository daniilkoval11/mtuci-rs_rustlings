// AsRef and AsMut allow for cheap reference-to-reference conversions.
// Read more about them at https://doc.rust-lang.org/std/convert/trait.AsRef.html
// and https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectively.
// Execute `rustlings hint as_ref_mut` or use the `hint` watch subcommand for a hint.
fn calculate_average(values: &[f64]) -> f64 {
    let total: f64 = values.iter().sum();
    total / values.len() as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_average() {
        let values = [1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(calculate_average(&values), 3.0);
    }
}
