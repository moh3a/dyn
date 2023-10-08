// https://www.hackerrank.com/challenges/append-and-delete/problem
// Append and Delete

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
You have two strings of lowercase English letters. You can perform two types of operations on the first string:
1. Append a lowercase English letter to the end of the string.
2. Delete the last character of the string. Performing this operation on an empty string results in an empty string.
*/

/*
 * Complete the 'appendAndDelete' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts following parameters:
 *  1. STRING s
 *  2. STRING t
 *  3. INTEGER k
 */

fn appendAndDelete(s: &str, t: &str, k: i32) -> String {
    let mut result = "No";

    let mut longest_string = 0;
    if s.len() > t.len() {
        longest_string = s.len();
    } else {
        longest_string = t.len();
    }

    let mut last_equal_index = 101;
    for i in 0..longest_string {
        if i == s.len() || i == t.len() {
            last_equal_index = i;
            break;
        } else if &s[0..=i].to_string() != &t[0..=i].to_string() {
            last_equal_index = i - 1;
            break;
        }
    }

    let least_num_steps = (s.len() + t.len() - 2 - (2 * last_equal_index)) as i32;
    let can_go_back_and_forth = k % 2 == (((t.len() + s.len()) % 2) as i32);

    if ((s.len() + t.len()) as i32) < k {
        result = "Yes";
    } else if last_equal_index == 101 && can_go_back_and_forth {
        result = "Yes";
    } else if last_equal_index == s.len()
        && ((t.len() - s.len() + 1) as i32) < k
        && can_go_back_and_forth
    {
        result = "Yes";
    } else if last_equal_index == t.len()
        && ((s.len() - t.len() + 1) as i32) < k
        && can_go_back_and_forth
    {
        result = "Yes";
    } else if least_num_steps == k {
        result = "Yes";
    } else {
        result = "No";
    }

    result.to_string()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let t = stdin_iterator.next().unwrap().unwrap();

    let k = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let result = appendAndDelete(&s, &t, k);

    writeln!(&mut fptr, "{}", result).ok();
}
