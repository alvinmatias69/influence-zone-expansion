use super::data::{Line, Point};

pub fn generate(query_point: &Point, interest_points: &Vec<Point>) -> Vec<Line> {
  let mut lines: Vec<Line> = Vec::new();
  for point in interest_points.into_iter() {
    lines.push(compute(&query_point, point));
  }
  lines
}

pub fn compute(point_1: &Point, point_2: &Point) -> Line {
  let m = -1.0 / calculate_slope(&point_1, &point_2);
  let c = calculate_c(m, &calculate_mid(&point_1, &point_2));
  Line { m, c }
}

fn calculate_slope(point_1: &Point, point_2: &Point) -> f64 {
  (point_2.y - point_1.y) / (point_2.x - point_1.x)
}

fn calculate_mid(point_1: &Point, point_2: &Point) -> Point {
  Point {
    x: (point_1.x + point_2.x) / 2.0,
    y: (point_1.y + point_2.y) / 2.0,
  }
}

fn calculate_c(slope: f64, mid_point: &Point) -> f64 {
  mid_point.y - mid_point.x * slope
}
