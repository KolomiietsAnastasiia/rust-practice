use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'diagonalDifference' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts 2D_INTEGER_ARRAY arr as parameter.
 *
 * https://www.hackerrank.com/challenges/diagonal-difference/problem
 */

fn diagonalDifference(arr: &[Vec<i32>]) -> i32 {
    let mut diag = 0;
    let mut diag2 = 0;

    let H = arr.len();
    let V = arr[0].len();

    for line in 0..H {
        for number in 0..V {

            if line == number {
                diag += arr[line][number];
            }

            if line == (H - 1) - number {
                diag2 += arr[line][number];
            }
        }
    }

    return (diag - diag2).abs();
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut arr: Vec<Vec<i32>> = Vec::with_capacity(n as usize);

    for i in 0..n as usize {
        arr.push(Vec::with_capacity(n as usize));

        arr[i] = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    let result = diagonalDifference(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}
