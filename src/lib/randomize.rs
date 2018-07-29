use rand;
use rand::Rng;

use super::data::Point;

pub fn point(bound: f64) -> Point {
  Point {
    x: rand::thread_rng().gen_range(0.0, bound),
    y: rand::thread_rng().gen_range(0.0, bound),
  }
}
