// https://www.hackerrank.com/challenges/manasa-and-stones/problem
// Manasa and Stones

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
Manasa is out on a hike with friends. She finds a trail of stones with numbers on them. She starts following the trail and notices that any two consecutive stones' numbers differ by one of two values. Legend has it that there is a treasure trove at the end of the trail. If Manasa can guess the value of the last stone, the treasure will be hers.
*/

/*
 * Complete the 'stones' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. INTEGER a
 *  3. INTEGER b
 */

fn stones(n: i32, a: i32, b: i32) -> Vec<i32> {
    let mut possibilities: Vec<i32> = Vec::new();

    for i in 0..n {
        let fpos = (b * (i as i32)) + (a * (n - (i as i32) - 1));
        if !possibilities.contains(&fpos) {
            possibilities.push(fpos);
        }
        let spos = (a * (i as i32)) + (b * (n - (i as i32) - 1));
        if !possibilities.contains(&spos) {
            possibilities.push(spos);
        }
    }

    possibilities.sort();
    possibilities
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let T = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    for _ in 0..T {
        let n = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .parse::<i32>()
            .unwrap();
        let a = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .parse::<i32>()
            .unwrap();
        let b = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .parse::<i32>()
            .unwrap();
        let result = stones(n, a, b);

        for i in 0..result.len() {
            write!(&mut fptr, "{}", result[i]).ok();

            if i != result.len() - 1 {
                write!(&mut fptr, " ").ok();
            }
        }

        writeln!(&mut fptr).ok();
    }
}
