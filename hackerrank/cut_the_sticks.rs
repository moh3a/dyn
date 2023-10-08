// https://www.hackerrank.com/challenges/cut-the-sticks/problem
// Cut the sticks

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
You are given a number of sticks of varying lengths. You will iteratively cut the sticks into smaller sticks, discarding the shortest pieces until there are none left. At each iteration you will determine the length of the shortest stick remaining, cut that length from each of the longer sticks and then discard all the pieces of that shortest length. When all the remaining sticks are the same length, they cannot be shortened so discard them.

Given the lengths of n sticks, print the number of sticks that are left before each iteration until there are none left.
*/

/*
 * Complete the 'cutTheSticks' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn cutTheSticks(arr: &[i32]) -> Vec<i32> {
    let mut stick_count = Vec::new();
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort();

    let mut current_length = sorted_arr[0];
    let mut current_index = 0;
    for _ in 0..=1000 {
        if sorted_arr[current_index] == 0 {
            let mut changed = false;
            for (i, v) in sorted_arr[current_index..sorted_arr.len()]
                .iter()
                .enumerate()
            {
                if *v != 0 {
                    current_length = *v;
                    current_index = i;
                    changed = true;
                    break;
                }
            }
            if changed == false {
                current_length = 0;
                current_index = sorted_arr.len() - 1;
                break;
            }
        }
        let mut count = 0;
        for i in current_index..sorted_arr.len() {
            if sorted_arr[i] - current_length >= 0 {
                sorted_arr[i] -= current_length;
                count += 1;
            }
        }
        stick_count.push(count);
    }

    stick_count
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let arr: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = cutTheSticks(&arr);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            writeln!(&mut fptr).ok();
        }
    }

    writeln!(&mut fptr).ok();
}
