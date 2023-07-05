// https://www.hackerrank.com/challenges/library-fine/problem
// Library Fine

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/* Given the expected and actual return dates for a library book, create a program that calculates the fine (if any). */

/*
 * Complete the 'libraryFine' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER d1
 *  2. INTEGER m1
 *  3. INTEGER y1
 *  4. INTEGER d2
 *  5. INTEGER m2
 *  6. INTEGER y2
 */

fn libraryFine(d1: i32, m1: i32, y1: i32, d2: i32, m2: i32, y2: i32) -> i32 {
    let mut fine = 0;
    if y1 > y2 {
        fine = 10000;
    } else if y1 < y2 {
        fine = 0;
    } else {
        if m1 > m2 {
            fine = 500 * (m1 - m2);
        } else if m1 < m2 {
            fine = 0;
        } else {
            if d1 > d2 {
                fine = 15 * (d1 - d2);
            }
        }
    }
    fine
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

    let d1 = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let m1 = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let y1 = first_multiple_input[2].trim().parse::<i32>().unwrap();

    let second_multiple_input: Vec<String> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let d2 = second_multiple_input[0].trim().parse::<i32>().unwrap();

    let m2 = second_multiple_input[1].trim().parse::<i32>().unwrap();

    let y2 = second_multiple_input[2].trim().parse::<i32>().unwrap();

    let result = libraryFine(d1, m1, y1, d2, m2, y2);

    writeln!(&mut fptr, "{}", result).ok();
}
