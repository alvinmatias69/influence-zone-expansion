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
use lib::common;

fn main() {
    // output::create_query_file();
    // let data_count: [usize; 8] = [1, 100, 500, 1000, 1500, 2000, 2500, 3000];
    // compute_object_k(&data_count);

    // output::create_time_file();
    // output::create_segment_file();
    // output::create_size_file();
    // generate_mul_start(100, 10, 100);

    // generate_mul(4, 500);

    // output::create_query_file();
    // compute_k(4000, 200, 10000);

    output::create_time_file();
    generate_mul_start_csv(0, 10, 100);
}

fn generate_mul_start_csv(start: usize, total: usize, multiplier: usize) {
    let mut query_point: Point;
    for count in 1..total + 1 {
        query_point = randomize::point(10.0);
        compute_partial(count * multiplier + start, &query_point);
    }
}

fn compute_partial(count: usize, query_point: &Point) {
    let bound = 10.0;
    let mut interest_points: Vec<Point> = Vec::new();

    for _ in 0..count {
        interest_points.push(randomize::point(bound));
    }

    output::create_folder(count);
    output::create_query_csv(count, &query_point);
    output::create_interest_csv(&interest_points);
    output::create_zone_csv(count);

    let start: SystemTime = SystemTime::now();
    influence_zone::compute_partial(&query_point, &interest_points, bound);
    let time: Duration = start.elapsed().unwrap();

    output::save_time(count, time);
}

fn generate_mul_start(start: usize, total: usize, multiplier: usize) {
    let mut query_point: Point;
    for count in 1..total + 1 {
        query_point = randomize::point(10.0);
        compute(count * multiplier + start, &query_point);
    }
}

fn generate_mul(total: usize, multiplier: usize) {
    let mut query_point: Point;
    for count in 1..total + 1 {
        query_point = randomize::point(10.0);
        compute(count * multiplier, &query_point);
    }
}

fn generate(total: usize) {
    let mut query_point: Point;
    for count in 2..total {
        query_point = randomize::point(10.0);
        compute(count, &query_point);
    }
}

fn compute_k(interest_points: usize, multiplier: usize, object: usize) {
    let bound = 10.0;
    let query_point: Point = randomize::point(bound);
    let zone = compute(interest_points, &query_point);
    let mut objects: Vec<Point> = Vec::new();
    let mut start: SystemTime;
    let mut time: Duration;
    let mut found: bool;
    let mut totally: bool = false;

    for k in 0..(interest_points / multiplier + 1) {
        objects.clear();
        for _ in 0..object {
            objects.push(randomize::point(bound));
        }

        time = Duration::new(0, 0);
        for object_data in objects.iter() {
            start = SystemTime::now();
            found = influence_zone::query(&query_point, &zone, &object_data, k * multiplier);
            totally = totally || found;
            time = time.checked_add(start.elapsed().unwrap()).unwrap();
        }

        output::save_query_time(k * multiplier, objects.len(), time);
    }
}

fn compute_object_k(data_count: &[usize; 8]) {
    let bound = 10.0;
    let query_point: Point = randomize::point(bound);
    let zone = compute(data_count[7], &query_point);
    let mut objects: Vec<Point> = Vec::new();
    let mut start: SystemTime;
    let mut time: Duration;
    let mut found: bool;
    let mut totally: bool = false;

    for object_count in 0..8 {
        objects.clear();

        for _ in 0..(data_count[object_count] + 1) * 1000 {
            objects.push(randomize::point(bound));
        }

        for k in 0..8 {
            time = Duration::new(0, 0);
            for object in objects.iter() {
                start = SystemTime::now();
                found = influence_zone::query(&query_point, &zone, &object, data_count[k] - 1);
                time = time.checked_add(start.elapsed().unwrap()).unwrap();
                totally = totally || found;
            }
            output::save_query_time(data_count[k], data_count[object_count] * 100, time);
        }
    }
}

fn compute(count: usize, query_point: &Point) -> Vec<Vec<Segment>> {
    let bound = 10.0;
    let mut interest_points: Vec<Point> = Vec::new();

    for _ in 0..count {
        interest_points.push(randomize::point(bound));
    }

    let start: SystemTime = SystemTime::now();
    let zone: Vec<Vec<Segment>> = influence_zone::compute(&query_point, &interest_points, bound);
    let time: Duration = start.elapsed().unwrap();

    let average: usize = common::average_segments(&zone);

    output::save_time(count, time);
    output::save_segment_average(count, average);
    output::save_file_size(count, average);
    // output::save_output(&query_point, &interest_points, &zone, &time);
    zone
}
