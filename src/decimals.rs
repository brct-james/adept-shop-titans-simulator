/// Rounds a f64 to 2 decimal places, as long as the float is not near the max value
pub fn round_to_2(float64: f64) -> f64 {
    return (float64 * 100.0).round() / 100.0;
}

/// Rounds an array of f64s to 2 decimal places
pub fn _round_array_of_len_4_to_2(f64_arr: [f64; 4]) -> [f64; 4] {
    let mut res = f64_arr.clone();
    for (i, entry) in f64_arr.iter().enumerate() {
        res[i] = round_to_2(*entry);
    }
    return res;
}
