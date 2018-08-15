use std::cmp::Ordering;
use super::data::{Line, Point, Segment};

pub fn generate(lines: &Vec<Line>, bound: f64) -> Vec<Vec<Point>> {
  let mut outer: usize = 0;
  let mut inner: usize;
  let mut points: Vec<Vec<Point>> = Vec::new();
  let mut point: Vec<Point> = Vec::new();
  let mut cleaned: Vec<Point> = Vec::new();

  while outer < lines.len() {
    inner = 0;
    point.clear();
    cleaned.clear();

    while inner < lines.len() {
      if inner != outer {
        point.push(compute(&lines[outer], &lines[inner]));
      }
      inner = inner + 1;
    }

    point.extend(compute_bound(&lines[outer], bound));
    cleaned.extend(sort(&remove_outside_bound(&point, bound)));
    points.push(cleaned.clone());
    outer = outer + 1;
  }

  points
}

pub fn generate_partial(lines: &Vec<Line>, bound: f64, index: usize) -> Vec<Point> {
  let mut intersections: Vec<Point> = Vec::new();

  for (inner, line) in lines.iter().enumerate() {
    if inner != index {
      intersections.push(compute(&lines[index], &line));
    }
  }

  intersections.extend(compute_bound(&lines[index], bound));

  let result = sort(&remove_outside_bound(&intersections, bound));
  result
}

pub fn compute(line_1: &Line, line_2: &Line) -> Point {
  let x: f64 = calculate_x(&line_1, &line_2);
  let y: f64 = line_1.m.mul_add(x, line_1.c);
  Point { x, y }
}

fn calculate_x(line_1: &Line, line_2: &Line) -> f64 {
  (line_2.c - line_1.c) / (line_1.m - line_2.m)
}

fn compute_bound(line: &Line, bound: f64) -> Vec<Point> {
  let mut points: Vec<Point> = Vec::new();

  let mut point: Point = Point { x: 0.0, y: line.c };
  points.push(point.clone());

  point = Point {
    x: bound,
    y: line.m.mul_add(bound, line.c),
  };
  points.push(point.clone());

  point = Point {
    x: -line.c / line.m,
    y: 0.0,
  };
  points.push(point.clone());

  point = Point {
    x: (bound - line.c) / line.m,
    y: bound,
  };
  points.push(point.clone());

  points
}

fn remove_outside_bound(points: &Vec<Point>, bound: f64) -> Vec<Point> {
  let mut cleaned: Vec<Point> = Vec::new();

  for point in points.into_iter() {
    if point.x >= 0.0 && point.x <= bound && point.y >= 0.0 && point.y <= bound {
      cleaned.push(point.clone());
    }
  }

  cleaned
}

fn sort(points: &Vec<Point>) -> Vec<Point> {
  let mut sorted = points.clone();
  sorted.sort_by(|a, b| {
    if a.x < b.x {
      return Ordering::Less;
    }

    if a.x > b.x {
      return Ordering::Greater;
    }

    if a.y < b.y {
      return Ordering::Less;
    }

    if a.y > b.y {
      return Ordering::Greater;
    }

    return Ordering::Equal;
  });

  sorted
}

pub fn line_segment(line: &Line, segment: &Segment) -> bool {
  let t: f64 = (line.c - segment.start.y + line.m * segment.start.x)
    / (segment.end.y - segment.start.y + line.m * (segment.start.x - segment.end.x));

  t > 0.0 && t < 1.0
}

pub fn segment(segment_1: &Segment, segment_2: &Segment) -> bool {
  let orientation_1: u32 =
    calculate_orientation(&segment_1.start, &segment_1.end, &segment_2.start);
  let orientation_2: u32 = calculate_orientation(&segment_1.start, &segment_1.end, &segment_2.end);
  let orientation_3: u32 =
    calculate_orientation(&segment_2.start, &segment_2.end, &segment_1.start);
  let orientation_4: u32 = calculate_orientation(&segment_2.start, &segment_2.end, &segment_1.end);

  orientation_1 != orientation_2 && orientation_3 != orientation_4
}

fn calculate_orientation(point_1: &Point, point_2: &Point, point_3: &Point) -> u32 {
  let val: f64 = (point_2.y - point_1.y) * (point_3.x - point_2.x)
    - (point_2.x - point_1.x) * (point_3.y - point_2.y);

  if val == 0.0 {
    0
  } else if val > 0.0 {
    1
  } else {
    2
  }
}
