// https://www.hackerrank.com/challenges/jumping-on-the-clouds/problem
// Jumping on the Clouds

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
There is a new mobile game that starts with consecutively numbered clouds. Some of the clouds are thunderheads and others are cumulus. The player can jump on any cumulus cloud having a number that is equal to the number of the current cloud plus 1 or 2. The player must avoid the thunderheads. Determine the minimum number of jumps it will take to jump from the starting postion to the last cloud. It is always possible to win the game.

For each game, you will get an array of clouds numbered 0 if they are safe or 1 if they must be avoided.

*/

/*
 * Complete the 'jumpingOnClouds' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY c as parameter.
 */

fn jumpingOnClouds(c: &[i32]) -> i32 {
    let mut jumps = -1;
    let mut index: usize = 0;
    while index < c.len() {
        jumps += 1;
        if index < c.len() - 2 && c[index + 2] == 0 {
            index += 2;
        } else {
            index += 1;
        }
    }
    jumps
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let c: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = jumpingOnClouds(&c);

    writeln!(&mut fptr, "{}", result).ok();
}
