use std::io::{self, BufRead};

/*
 * Complete the 'plusMinus' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 *
 * https://www.hackerrank.com/challenges/plus-minus/problem
 */

fn plusMinus(arr: &[i32]) {
    let length = arr.len() as f64;
    let mut positive = 0;
    let mut negative = 0;
    let mut zero = 0;

    for number in arr.iter() {
        if *number > 0 {positive += 1}
        else if *number < 0 {negative += 1}
        else {zero += 1}

    }

    print!(
        "{:.6}\n{:.6}\n{:.6}",
        positive as f64 / length,
        negative as f64 / length,
        zero as f64 / length
    );
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    plusMinus(&arr);
}
