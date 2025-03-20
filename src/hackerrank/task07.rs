use std::io::{self, BufRead};

/*
 * Complete the 'miniMaxSum' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 *
 * https://www.hackerrank.com/challenges/mini-max-sum/problem
 */

fn miniMaxSum(arr: &[i32]) {
    let mut main_sum = arr.iter().sum::<i32>();
    let mut min = arr[0];
    let mut max = 0;

    for number in arr {
        if number > &max {max = *number;}
        if number < &min {min = *number;}
    }

    print!("{} {}", main_sum - max, main_sum - min);

}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    miniMaxSum(&arr);
}
