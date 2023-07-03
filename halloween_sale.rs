// https://www.hackerrank.com/challenges/halloween-sale/problem
// Halloween Sale

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
You wish to buy video games from the famous online video game store Mist.

Usually, all games are sold at the same price, p dollars. However, they are planning to have the seasonal Halloween Sale next month in which you can buy games at a cheaper price. Specifically, the first game will cost d dollars, and every subsequent game will cost p dollars less than the previous one. This continues until the cost becomes less than or equal to m dollars, after which every game will cost m dollars. How many games can you buy during the Halloween Sale?
 */

/*
 * Complete the 'howManyGames' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER p
 *  2. INTEGER d
 *  3. INTEGER m
 *  4. INTEGER s
 */

fn howManyGames(p: i32, d: i32, m: i32, s: i32) -> i32 {
    let mut price = p;
    let mut budget = s;
    let mut count = 0;
    while budget > 0 {
        if budget - price < 0 {
            break;
        }
        budget -= price;
        if price - d > m {
            price -= d;
        } else {
            price = m;
        }
        count += 1;
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

    let p = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let d = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let m = first_multiple_input[2].trim().parse::<i32>().unwrap();

    let s = first_multiple_input[3].trim().parse::<i32>().unwrap();

    let answer = howManyGames(p, d, m, s);

    writeln!(&mut fptr, "{}", answer).ok();
}
