use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'timeConversion' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 *
 * https://www.hackerrank.com/challenges/time-conversion/problem
 */

fn timeConversion(s: &str) -> String {
    let mut hh: i32 = s.get(0..2).unwrap().parse().unwrap();
    let mm = s.get(3..5).unwrap();
    let ss = s.get(6..8).unwrap();
    let M = s.get(8..10).unwrap();

    if M == "AM" {
        if hh == 12 {
            hh = 0
        }
    }
    else {
        if hh != 12 {
            hh = 12 + hh
        }
    }

    return format!("{:02}:{}:{}", hh, mm, ss);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = timeConversion(&s);

    writeln!(&mut fptr, "{}", result).ok();
}
