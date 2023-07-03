// https://www.hackerrank.com/challenges/one-week-preparation-kit-mini-max-sum/problem
// Mini-Max Sum

use std::io::{self, BufRead};

/*
Given five positive integers, find the minimum and maximum values that can be calculated by summing exactly four of the five integers. Then print the respective minimum and maximum values as a single line of two space-separated long integers.
 */

/*
 * Complete the 'miniMaxSum' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn miniMaxSum(arr: &[i32]) {
    let mut newa = arr.to_vec();
    let mut min: i64 = 0;
    let mut max: i64 = 0;
    newa.sort();
    for i in 0..5 {
        if i < 4 {
            min += newa[i] as i64;
        }
        if i > 0 {
            max += newa[i] as i64;
        }
    }
    println!("{} {}", min, max);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    miniMaxSum(&arr);
}
