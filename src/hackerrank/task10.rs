use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'gradingStudents' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts INTEGER_ARRAY grades as parameter.
 *
 * https://www.hackerrank.com/challenges/grading/problem
 */

fn gradingStudents(grades: &[i32]) -> Vec<i32> {
    let mut new_grades = Vec::new();

    for &grade in grades {
        if grade < 38 {
            new_grades.push(grade);
            continue;
        }
        if grade >= 100 {
            new_grades.push(100);
            continue;
        }

        let mut rounded_grade = grade;
        for i in 0..3 {
            if (grade + i) % 5 == 0 {
                rounded_grade = grade + i;
                break;
            }
        }

        if rounded_grade - grade < 3 {
            new_grades.push(rounded_grade);
        } else {
            new_grades.push(grade);
        }
    }

    new_grades
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let grades_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut grades: Vec<i32> = Vec::with_capacity(grades_count as usize);

    for _ in 0..grades_count {
        let grades_item = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
        grades.push(grades_item);
    }

    let result = gradingStudents(&grades);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            writeln!(&mut fptr).ok();
        }
    }

    writeln!(&mut fptr).ok();
}
