#[macro_use]
extern crate serde_derive;

extern crate rand;
extern crate serde_json;

use std::time::{Duration, SystemTime};

mod lib;
use lib::data::{Point, Segment};
use lib::influence_zone;
use lib::randomize;
use lib::output;

fn main() {
    generate_zone(1001);
    // let object: Point = randomize::point(bound);

    // let value: usize = influence_zone::query(&query_point, &zone, &object);
    // println!("{}", value);
    // println!("{:?}", start.elapsed().unwrap());
}

fn generate_zone(count_total: usize) {
    let bound = 10.0;

    let mut query_point: Point;
    let mut interest_points: Vec<Point> = Vec::new();
    let mut start: SystemTime;
    let mut time: Duration;
    let mut zone: Vec<Vec<Segment>>;

    output::create_time_file();

    for count in 2..count_total {
        query_point = randomize::point(bound);
        interest_points.clear();

        for _ in 0..count {
            interest_points.push(randomize::point(bound));
        }

        start = SystemTime::now();
        zone = influence_zone::compute(&query_point, &interest_points, bound);
        time = start.elapsed().unwrap();

        output::save_output(&query_point, &interest_points, &zone, &time);
        output::save_time(count, time)
    }
}
