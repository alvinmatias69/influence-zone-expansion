use super::data::{Point, Segment};

pub fn generate(points: &Vec<Vec<Point>>) -> Vec<Segment> {
  let mut segments: Vec<Segment> = Vec::new();
  for (index, point) in points.iter().enumerate() {
    segments.extend(compute(&point, index));
  }
  segments
}

pub fn compute(points: &Vec<Point>, from: usize) -> Vec<Segment> {
  let mut index = 0;
  let mut segments: Vec<Segment> = Vec::new();

  if points.len() > 0 {
    while index < points.len() - 1 {
      segments.push(Segment {
        start: points[index].clone(),
        end: points[index + 1].clone(),
        from,
      });
      index = index + 1;
    }
  }

  segments
}

pub fn mid_point(segment: &Segment) -> Point {
  let x: f64 = (segment.start.x + segment.end.x) / 2.0;
  let y: f64 = (segment.start.y + segment.end.y) / 2.0;
  Point { x, y }
}
