pub fn round_to(number: f64, precision: i32) -> f64 {
  let local_precision: f64 = (10.0 as f64).powi(precision);
  let int_number = (number * local_precision).floor() as i64;
  (int_number as f64) / local_precision
}
