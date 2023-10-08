// https://www.hackerrank.com/challenges/one-week-preparation-kit-time-conversion/problem
// Time Conversion

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'timeConversion' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

fn timeConversion(s: &str) -> String {
    let hour = &s[0..2].parse::<i32>().unwrap();
    let mut new_hour = *hour;
    if &s[8..] == "PM" {
        if *hour != 12 {
            new_hour += 12;
        }
    } else if *hour == 12 {
        new_hour = 0;
    }

    let mut str_hour = new_hour.to_string();
    if new_hour < 10 {
        str_hour = "0".to_owned() + &str_hour;
    }

    str_hour + &s[2..8]
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = timeConversion(&s);

    writeln!(&mut fptr, "{}", result).ok();
}
