mod sorting_algorithm;
use rand::prelude::*;
use crate::sorting_algorithm::quicksort::quicksort::Quicksort;

fn main() {
    let mut rng = rand::thread_rng();
    let mut test:[i32;50] = [0;50];
    for n in 0..test.len() {
        test[n] = rng.gen();
        println!("{:?}, ", test[n]);
    }
    println!();
    println!("starting quicksort!");
    test.quicksort();
    println!("quicksort finished!");
    for n in 0..test.len() {
        println!("{:?}, ", test[n]);
    }
}
