use super::data::Segment;

// pub fn round_to(number: f64, precision: i32) -> f64 {
//   let local_precision: f64 = (10.0 as f64).powi(precision);
//   let int_number = (number * local_precision).floor() as i64;
//   (int_number as f64) / local_precision
// }

pub fn average_segments(influence_zone: &Vec<Vec<Segment>>) -> usize {
  let mut count: usize = 0;
  let mut segment_count: usize = 0;
  for zone in influence_zone.iter() {
    if zone.len() > 0 {
      segment_count = segment_count + 1;
    }
    count = count + zone.len();
  }

  count / segment_count
}
