use ndarray::Array1;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
/// Cumulative trapezoidal integration for a 1D array.
/// Returns an Array1 of integrated values, assuming initial value is the same as Array1.
pub fn cumtrapz(y: &Array1<f64>, dt: f64) -> Array1<f64> {
    let mut out = Array1::zeros(y.len());
    out[0] = y[0];
    for i in 1..y.len() {
        out[i] = out[i - 1] + 0.5 * (y[i] + y[i - 1]) * dt;
    }
    out
}
/// Load a single-column file into a Vec<f64>.
pub fn load_column_file<P: AsRef<Path>>(path: P) -> Vec<f64> {
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map_while(Result::ok)
        .filter_map(|line| line.trim().parse::<f64>().ok())
        .collect()
}

/// Helper to get min and max of a slice
pub fn min_max(data: &[f64]) -> (f64, f64) {
    let min = data.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    (min, max)
}
