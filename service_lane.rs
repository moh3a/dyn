// https://www.hackerrank.com/challenges/service-lane/problem
// Service Lane

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
A driver is driving on the freeway. The check engine light of his vehicle is on, and the driver wants to get service immediately. Luckily, a service lane runs parallel to the highway. It varies in width along its length.

You will be given an array of widths at points along the road (indices), then a list of the indices of entry and exit points. Considering each entry and exit point pair, calculate the maximum size vehicle that can travel that segment of the service lane safely.

 */

/*
 * Complete the 'serviceLane' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. 2D_INTEGER_ARRAY cases
 */

fn serviceLane(n: i32, width: Vec<i32>, cases: &[Vec<i32>]) -> Vec<i32> {
    let mut result = Vec::new();
    for case in cases.iter() {
        result.push(
            *width[(case[0] as usize)..=(case[1] as usize)]
                .iter()
                .min()
                .unwrap(),
        );
    }
    result.to_vec()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let t = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let width: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let mut cases: Vec<Vec<i32>> = Vec::with_capacity(t as usize);

    for i in 0..t as usize {
        cases.push(Vec::with_capacity(2_usize));

        cases[i] = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    let result = serviceLane(n, width, &cases);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            writeln!(&mut fptr).ok();
        }
    }

    writeln!(&mut fptr).ok();
}
