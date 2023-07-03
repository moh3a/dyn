// https://www.hackerrank.com/challenges/kaprekar-numbers/problem
// Modified Kaprekar Numbers

use std::io::{self, BufRead};

/*
A modified Kaprekar number is a positive whole number with a special property. If you square it, then split the number into two integers and sum those integers, you have the same value you started with.
 */

/*
 * Complete the 'kaprekarNumbers' function below.
 *
 * The function accepts following parameters:
 *  1. INTEGER p
 *  2. INTEGER q
 */

fn kaprekarNumbers(p: i32, q: i32) {
    let mut result = String::new();

    for i in (p as i64)..=(q as i64) {
        let original_str = i.to_string();
        let square_str = (i * i).to_string();
        let (mut s1, mut s2): (&str, &str) = ("", "");
        if square_str.len() == original_str.len() * 2 {
            (s1, s2) = square_str.split_at(original_str.len());
        } else if square_str.len() == (original_str.len() * 2 - 1) {
            (s1, s2) = square_str.split_at(original_str.len() - 1);
        }
        let first_half = match s1.parse::<i64>() {
            Ok(n) => n,
            _ => 0,
        };
        let second_half = match s2.parse::<i64>() {
            Ok(n) => n,
            _ => 0,
        };
        if first_half + second_half == i {
            if result.len() != 0 {
                result.push_str(" ");
            }
            result.push_str(&i.to_string());
        }
    }

    if result.len() == 0 {
        result = "INVALID RANGE".to_string();
    }

    println!("{}", result);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let p = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let q = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    kaprekarNumbers(p, q);
}
