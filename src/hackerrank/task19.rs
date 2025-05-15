use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'sockMerchant' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. INTEGER_ARRAY ar
 *
 *  https://www.hackerrank.com/challenges/sock-merchant/problem
 */

fn sockMerchant(n: i32, ar: &[i32]) -> i32 {
    let mut count = 0;
    let mut used = vec![false; n as usize];

    for i in 0..n as usize {
        if used[i] {
            continue
        }

        for j in (i + 1)..n as usize {
            if !used[j] && ar[i] == ar[j] {
                count += 1;
                used[i] = true;
                used[j] = true;
                break;
            }
        }
    }

    count
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let ar: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = sockMerchant(n, &ar);

    writeln!(&mut fptr, "{}", result).ok();
}
