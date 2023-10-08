// https://www.hackerrank.com/challenges/repeated-string/problem
// Repeated String

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
There is a string, s, of lowercase English letters that is repeated infinitely many times. Given an integer, n, find and print the number of letter a's in the first n letters of the infinite string.


*/

/*
 * Complete the 'repeatedString' function below.
 *
 * The function is expected to return a LONG_INTEGER.
 * The function accepts following parameters:
 *  1. STRING s
 *  2. LONG_INTEGER n
 */

fn repeatedString(s: &str, n: i64) -> i64 {
    let s_chars: Vec<char> = s.chars().collect();
    let mut indexes = Vec::new();
    let mut count_in_s = 0;
    for (i, c) in s_chars.iter().enumerate() {
        if *c == 'a' {
            indexes.push(i);
            count_in_s += 1;
        }
    }

    let repitions = n / (s_chars.len() as i64);
    let mut last_repition = n - (repitions * s_chars.len() as i64);
    let mut last_count_in_s = 0;
    for i in indexes.iter() {
        if *i < last_repition as usize {
            last_count_in_s += 1;
        }
    }

    ((count_in_s as i64) * repitions) + last_count_in_s
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i64>()
        .unwrap();

    let result = repeatedString(&s, n);

    writeln!(&mut fptr, "{}", result).ok();
}
