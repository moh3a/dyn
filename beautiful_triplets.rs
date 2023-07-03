// https://www.hackerrank.com/challenges/beautiful-triplets/problem
// Beautiful Triplets

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'beautifulTriplets' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER d
 *  2. INTEGER_ARRAY arr
 */

fn beautifulTriplets(d: i32, arr: &[i32]) -> i32 {
    let mut count = 0;
    for i in 0..arr.len() {
        if i < arr.len() - 2 {
            for j in (i + 1)..arr.len() {
                let mut found_k = false;
                if j < arr.len() - 1 && arr[j] == arr[i] + d {
                    for k in (j + 1)..arr.len() {
                        if arr[k] == arr[j] + d {
                            count += 1;
                            found_k = true;
                            break;
                        }
                    }
                }
                if found_k {
                    break;
                }
            }
        }
    }
    count
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

    let d = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = beautifulTriplets(d, &arr);

    writeln!(&mut fptr, "{}", result).ok();
}
