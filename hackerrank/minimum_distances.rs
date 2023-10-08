// https://www.hackerrank.com/challenges/minimum-distances/problem
// Minimum Distances

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

// The distance between two array values is the number of indices between them. Given a, find the minimum distance between any pair of equal elements in the array. If no such value exists, return -1.

/*
 * Complete the 'minimumDistances' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY a as parameter.
 */

fn minimumDistances(a: &[i32]) -> i32 {
    let mut min = 1000;
    for i in 0..a.len() {
        let mut duplicate = false;
        if i < a.len() - 1 {
            for j in (i + 1)..a.len() {
                if a[i] == a[j] && ((j - i) as i32) < min {
                    min = (j - i) as i32;
                    duplicate = true;
                    break;
                }
            }
        }
        if duplicate {
            continue;
        }
    }
    if min == 1000 {
        min = -1;
    }
    min
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

    let a: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = minimumDistances(&a);

    writeln!(&mut fptr, "{}", result).ok();
}
