/// Rounds a f64 to 2 decimal places, as long as the float is not near the max value
pub fn round_to_2(float64: f64) -> f64 {
    return (float64 * 100.0).round() / 100.0;
}
