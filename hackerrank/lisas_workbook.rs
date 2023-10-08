// https://www.hackerrank.com/challenges/lisa-workbook/problem
// Lisa's Workbook

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
Lisa just got a new math workbook. A workbook contains exercise problems, grouped into chapters. Lisa believes a problem to be special if its index (within a chapter) is the same as the page number where it's located. The format of Lisa's book is as follows:

- There are n chapters in Lisa's workbook, numbered from 1 to n.
- The ith chapter has arr[i] problems, numbered from 1 to arr[i].
- Each page can hold up to k problems. Only a chapter's last page of exercises may contain fewer than k problems.
- Each new chapter starts on a new page, so a page will never contain problems from more than one chapter.
- The page number indexing starts at 1.

Given the details for Lisa's workbook, can you count its number of special problems?
 */

/*
 * Complete the 'workbook' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. INTEGER k
 *  3. INTEGER_ARRAY arr
 */

fn workbook(n: i32, k: i32, arr: &[i32]) -> i32 {
    let mut special_count = 0;
    let mut current_page = 0;
    for chapter_index in 1..=n {
        current_page += 1;
        let mut new_page = 0;
        for problems in 1..=arr[(chapter_index - 1) as usize] {
            if (problems - (k * new_page)) > k {
                new_page += 1;
                current_page += 1;
            }
            if problems == current_page {
                special_count += 1;
            }
        }
    }
    special_count
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

    let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = workbook(n, k, &arr);

    writeln!(&mut fptr, "{}", result).ok();
}
