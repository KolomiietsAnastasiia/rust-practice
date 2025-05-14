use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::rng();
    let mut vec = Vec::new();

    for _ in 0..n {
        let num = rng.random_range(10..=99);
        vec.push(num);
    }

    vec
}

fn min_adjacent_sum(data: &[i32]) -> usize {
    let mut min_index = 0;
    let mut min_sum = i32::MAX;

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_index = i;
        }
    }

    min_index
}

fn print_vector_min_pair(data: &[i32], min_index: usize) {
    print!("indexes:");
    for i in 0..data.len() {
        print!("{:3}", i);
    }
    println!();

    print!("data:   ");
    for val in data {
        print!("{:3}", val);
    }
    println!();


    print!("        ");
    for i in 0..data.len() {
        if i == min_index {
            print!(" \\__");
        } else if i == min_index + 1 {
            print!("_/");
            break;
        } else {
            print!("   ");
        }
    }
    println!();

    println!("min adjacent sum = {}+{} = {} | at indexes: {}, {}",data[min_index], data[min_index + 1], data[min_index] + data[min_index + 1], min_index, min_index + 1    );
}

#[test]
fn test() {
    let data = gen_random_vector(11);
    let min_index = min_adjacent_sum(&data);

    print_vector_min_pair(&data, min_index);
}

// OUTPUT examples:
//
//1.
// indexes:  0  1  2  3  4  5  6  7  8  9 10
// data:    41 95 86 48 74 62 51 24 19 97 69
//                               \___/
// min adjacent sum = 24+19 = 43 | at indexes: 7, 8
//
//2.
// indexes:  0  1  2  3  4  5  6  7  8  9 10
// data:    60 52 11 57 43 21 31 66 18 67 28
//                         \___/
// min adjacent sum = 21+31 = 52 | at indexes: 5, 6
//
//3.
// indexes:  0  1  2  3  4  5  6  7  8  9 10
// data:    73 57 74 97 87 71 13 20 47 51 43
//                            \___/
// min adjacent sum = 13+20 = 33 | at indexes: 6, 7
