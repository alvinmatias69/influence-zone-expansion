use serde_json;

use std::fs::File;
use std::io::Write;
use std::time::Duration;
use std::fs::OpenOptions;

use super::data::{Point, Segment};

#[derive(Serialize)]
struct Output {
  query_point: Point,
  interest_points: Vec<Point>,
  influence_zone: Vec<Vec<Segment>>,
  time: Duration,
}

pub fn save_output(
  query_point: &Point,
  interest_points: &Vec<Point>,
  influence_zone: &Vec<Vec<Segment>>,
  time: &Duration,
) {
  let output: Output = Output {
    query_point: query_point.clone(),
    interest_points: interest_points.clone(),
    influence_zone: influence_zone.clone(),
    time: *time,
  };

  let serialized_output = serde_json::to_string(&output).unwrap();

  let mut title: String = String::new();
  title.push_str("./output/");
  title.push_str(&interest_points.clone().len().to_string());
  title.push_str(".json");

  let mut file = File::create(title).expect("Unable to create data file");
  file
    .write_all(serialized_output.as_bytes())
    .expect("Unable to write to file");
}

pub fn create_time_file() {
  let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .open("./time.csv")
    .unwrap();

  if let Err(e) = writeln!(file, "interest points,seconds,nano") {
    eprintln!("Couldn't write to file: {}", e);
  }
}

pub fn save_time(count: usize, time: Duration) {
  let mut file = OpenOptions::new().append(true).open("./time.csv").unwrap();

  if let Err(e) = writeln!(file, "{},{},{}", count, time.as_secs(), time.subsec_nanos()) {
    eprintln!("Couldn't write to file: {}", e);
  }
}

pub fn create_segment_file() {
  let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .open("./segment.csv")
    .unwrap();

  if let Err(e) = writeln!(file, "interest points,segment") {
    eprintln!("Couldn't write to file: {}", e);
  }
}

pub fn save_segment_average(interests: usize, average: usize) {
  let mut file = OpenOptions::new()
    .append(true)
    .open("./segment.csv")
    .unwrap();

  if let Err(e) = writeln!(file, "{},{}", interests, average) {
    eprintln!("Couldn't write to file: {}", e);
  }
}

pub fn create_query_file() {
  let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .open("./query.csv")
    .unwrap();

  if let Err(e) = writeln!(file, "k,objects,second,nano") {
    eprintln!("Couldn't write to file: {}", e);
  }
}

pub fn save_query_time(k: usize, objects: usize, time: Duration) {
  let mut file = OpenOptions::new().append(true).open("./query.csv").unwrap();

  if let Err(e) = writeln!(
    file,
    "{},{},{},{}",
    k,
    objects,
    time.as_secs(),
    time.subsec_nanos()
  ) {
    eprintln!("Couldn't write to file: {}", e);
  }
}
