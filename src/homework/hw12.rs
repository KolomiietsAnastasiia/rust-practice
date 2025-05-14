use rand::Rng;

fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = rand::rng();
    let mut vec = Vec::new();

    for _ in 0..n {
        let num = rng.random_range(1..=100);
        vec.push(num);
    }

    vec
}

fn count_permutation(shipments: &Vec<u32>) -> i32 {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;

    if total % n != 0 {
        return -1;
    }

    let avg = total / n;
    let mut moves = 0;

    for &weight in shipments {
        if weight > avg {
            moves += weight - avg;
        }
    }

    moves as i32
}

#[test]
fn test() {
    let totest = [
        (vec![1, 1, 1, 1, 6], 4),
        (vec![8, 2, 2, 4, 4], 4),
        (vec![9, 3, 7, 2, 9], 7),
    ];

    for (ships, expected) in totest {
        println!("{:?}", ships);
        println!("Expected:{}  Got:{}",expected , count_permutation(&ships));

        assert_eq!(count_permutation(&ships), expected);

    }
}

// OUTPUT
// [1, 1, 1, 1, 6]
// Expected:4  Got:4
// [8, 2, 2, 4, 4]
// Expected:4  Got:4
// [9, 3, 7, 2, 9]
// Expected:7  Got:7