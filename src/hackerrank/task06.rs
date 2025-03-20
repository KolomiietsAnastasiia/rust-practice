use std::io::{self, BufRead};

/*
 * Complete the 'staircase' function below.
 *
 * The function accepts INTEGER n as parameter.
 *
 * https://www.hackerrank.com/challenges/staircase/problem
 */

fn staircase(n: i32) {
    for y in 0..n {
        for x in 0..n {
            let sym = if (x >= n - 1 - y) {'#'} else {' '};
            print!("{}", sym);
        }
        print!("\n");
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    staircase(n);
}
