#[macro_use]
extern crate serde_derive;

extern crate rand;
extern crate serde_json;

use std::time::SystemTime;

mod lib;
use lib::data::{Point, Segment};
use lib::influence_zone;
use lib::randomize;
use lib::output;

fn main() {
    output::create_time_file();
    generate_mul(21, 500)
    // generate(1001);
    // compute(2000);
    // let object: Point = randomize::point(bound);

    // let value: usize = influence_zone::query(&query_point, &zone, &object);
    // println!("{}", value);
    // println!("{:?}", start.elapsed().unwrap());
}

fn generate_mul(total: usize, multiplier: usize) {
    for count in 1..total {
        compute(count * multiplier);
    }
}

fn generate(total: usize) {
    for count in 2..total {
        compute(count);
    }
}

fn compute(count: usize) {
    let bound = 10.0;
    let query_point: Point = randomize::point(bound);
    let mut interest_points: Vec<Point> = Vec::new();

    for _ in 0..count {
        interest_points.push(randomize::point(bound));
    }

    let start = SystemTime::now();
    let zone: Vec<Vec<Segment>> = influence_zone::compute(&query_point, &interest_points, bound);
    let time = start.elapsed().unwrap();

    output::save_output(&query_point, &interest_points, &zone, &time);
    output::save_time(count, time);
}
