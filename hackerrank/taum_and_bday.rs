// https://www.hackerrank.com/challenges/taum-and-bday/problem
// Taum and B'day

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
Taum is planning to celebrate the birthday of his friend, Diksha. There are two types of gifts that Diksha wants from Taum: one is black and the other is white. To make her happy, Taum has to buy b black gifts and w white gifts.

- The cost of each black gift is bc units.
- The cost of every white gift is wc units.
- The cost to convert a black gift into white gift or vice versa is z units.
D
etermine the minimum cost of Diksha's gifts.
 */

/*
 * Complete the 'taumBday' function below.
 *
 * The function is expected to return a LONG_INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER b
 *  2. INTEGER w
 *  3. INTEGER bc
 *  4. INTEGER wc
 *  5. INTEGER z
 */

fn taumBday(b: i32, w: i32, bc: i32, wc: i32, z: i32) -> i64 {
    let mut min_cost: i64 = 0;
    let black = b as i64;
    let white = w as i64;
    let black_cost = bc as i64;
    let white_cost = wc as i64;
    let conversion_cost = z as i64;

    if black == white {
        if black_cost == white_cost {
            min_cost = black * black_cost + white * white_cost;
        } else if black_cost > white_cost {
            min_cost = black * (white_cost + conversion_cost) + white * white_cost;
        } else {
            min_cost = black * black_cost + white * (black_cost + conversion_cost);
        }
    } else if black_cost + conversion_cost < white_cost {
        min_cost = (white + black) * black_cost + white * conversion_cost;
    } else if white_cost + conversion_cost < black_cost {
        min_cost = (white + black) * white_cost + black * conversion_cost;
    } else {
        min_cost = black * black_cost + white * white_cost;
    }

    min_cost
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

        let b = first_multiple_input[0].trim().parse::<i32>().unwrap();

        let w = first_multiple_input[1].trim().parse::<i32>().unwrap();

        let second_multiple_input: Vec<String> = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .split(' ')
            .map(|s| s.to_string())
            .collect();

        let bc = second_multiple_input[0].trim().parse::<i32>().unwrap();

        let wc = second_multiple_input[1].trim().parse::<i32>().unwrap();

        let z = second_multiple_input[2].trim().parse::<i32>().unwrap();

        let result = taumBday(b, w, bc, wc, z);

        writeln!(&mut fptr, "{}", result).ok();
    }
}
