#[derive(Clone, Debug, Serialize)]
pub struct Point {
  pub x: f64,
  pub y: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct Segment {
  pub start: Point,
  pub end: Point,
  pub from: usize,
}

#[derive(Clone, Debug)]
pub struct Line {
  pub m: f64,
  pub c: f64,
}
