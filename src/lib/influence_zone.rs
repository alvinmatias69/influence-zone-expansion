use super::data::{Line, Point, Segment};
use super::perpendicular_bisector;
use super::intersection;
use super::segment;

pub fn compute(query_point: &Point, interest_points: &Vec<Point>, bound: f64) -> Vec<Vec<Segment>> {
  let bisector: Vec<Line> = perpendicular_bisector::generate(&query_point, &interest_points);
  let intersect: Vec<Vec<Point>> = intersection::generate(&bisector, bound);
  let segments: Vec<Segment> = segment::generate(&intersect);
  let mut query_segment: Segment;
  let mut count: usize;
  let mut zone: Vec<Vec<Segment>> = Vec::with_capacity(interest_points.len() + 1);

  for _ in 0..(interest_points.len() + 1) {
    zone.push(Vec::new());
  }

  for cur_segment in segments.into_iter() {
    query_segment = Segment {
      start: query_point.clone(),
      end: segment::mid_point(&cur_segment),
      from: cur_segment.from,
    };

    count = count_intersection(&query_segment, &bisector);

    zone[count].push(cur_segment.clone());
  }

  zone
}

fn count_intersection(segment: &Segment, bisector: &Vec<Line>) -> usize {
  let mut count: usize = 0;
  for (index, line) in bisector.iter().enumerate() {
    if index != segment.from && intersection::line_segment(&line, &segment) {
      count = count + 1;
    }
  }
  count
}

pub fn query(query_point: &Point, zone: &Vec<Vec<Segment>>, object: &Point) -> usize {
  let mut intersect: bool = true;
  let mut k: usize = 0;

  let query_segment: Segment = Segment {
    start: query_point.clone(),
    end: object.clone(),
    from: 0,
  };

  while intersect && k < zone.len() {
    intersect = false;
    for item in &zone[k] {
      if intersection::segment(&query_segment, &item) {
        intersect = true;
      }
    }
    k = k + 1;
  }

  k
}
