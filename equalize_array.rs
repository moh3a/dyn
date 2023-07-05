// https://www.hackerrank.com/challenges/equality-in-a-array/problem
// Equalize the Array

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
Given an array of integers, determine the minimum number of elements to delete to leave only elements of equal value.
 */

/*
 * Complete the 'equalizeArray' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn equalizeArray(arr: &[i32]) -> i32 {
  let mut new_arr = vec![0; (arr.iter().max().unwrap() + 1) as usize];
  for n in arr.iter() {
    new_arr[*n as usize] += 1;
  }

  let max = new_arr.iter().max().unwrap();
  let sum: i32 = new_arr.iter().sum();
  sum - max
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = equalizeArray(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}
