use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'migratoryBirds' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY arr as parameter.
 *
 * https://www.hackerrank.com/challenges/migratory-birds/problem
 */

fn migratoryBirds(arr: &[i32]) -> i32 {
    let mut types = vec![0; 6];

    for bird in arr {
        types[*bird as usize] += 1;
    }

    let mut max_count = 0;
    let mut bird_type = 0;

    for i in 1..=5 {
        if types[i] > max_count {
            max_count = types[i];
            bird_type = i;
        }
    }

    bird_type as i32
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _arr_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = migratoryBirds(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}
