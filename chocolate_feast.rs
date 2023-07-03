// https://www.hackerrank.com/challenges/chocolate-feast/problem
// Chocolate Feast

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

// Little Bobby loves chocolate. He frequently goes to his favorite 5&10 store, Penny Auntie, to buy them. They are having a promotion at Penny Auntie. If Bobby saves enough wrappers, he can turn them in for a free chocolate.

/*
 * Complete the 'chocolateFeast' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. INTEGER c
 *  3. INTEGER m
 */

fn chocolateFeast(n: i32, c: i32, m: i32) -> i32 {
    let mut chocolates_eaten = n / c;
    let mut wrappers = n / c;

    while wrappers >= m {
        wrappers -= m;
        wrappers += 1;
        chocolates_eaten += 1;
    }

    chocolates_eaten
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let t = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    for _ in 0..t {
        let first_multiple_input: Vec<String> = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .split(' ')
            .map(|s| s.to_string())
            .collect();

        let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

        let c = first_multiple_input[1].trim().parse::<i32>().unwrap();

        let m = first_multiple_input[2].trim().parse::<i32>().unwrap();

        let result = chocolateFeast(n, c, m);

        writeln!(&mut fptr, "{}", result).ok();
    }
}
