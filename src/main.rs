use rand::Rng;
use std::time::SystemTime;

use charliesort::sort_vec;

fn main() {
    for size in [100, 10000, 1000000, 10000000] {
        let mut v = Vec::<i32>::new();
        for _ in 0..size {
            v.push(rand::thread_rng().gen());
        }

        // time it
        let start = SystemTime::now();
        sort_vec(&mut v);
        let end = SystemTime::now();
        let duration = end.duration_since(start).unwrap().as_micros();

        // verify n * log(n) time complexity
        let n_log_n = duration as f64 / size as f64 / (size as f64).ln();
        println!("{} took {} us | ratio {n_log_n}", size, duration);

        // verify sort was successful
        for i in 1..v.len() {
            if v[i] < v[i - 1] {
                panic!("didn't sort correct for case {}", size);
            }
        }
    }
    // println!("{v:?}");
}
