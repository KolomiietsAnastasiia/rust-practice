fn find_max_par(s: String) -> i32 {
    let mut count = 0;
    let mut parts = 0;

    for symbol in s.chars() {
        match symbol {
            'L' => count += 1,
            'R' => count -= 1,
            _ => continue
        }
        if count == 0 {
            parts += 1
        }
    }

    parts
}

#[test]
fn test() {
    let to_test = [
        ("", 0),
        ("RL", 1),
        ("LR", 1),
        ("LRLR", 2),
        ("LLRLRR", 1),
        ("LLRLRLRLRLRLRR", 1),
        ("RLRRLLRLRRLL", 4),
        ("RLLLRRRLLR", 4)
    ];
    for (input, expected) in to_test.iter() {
        let real = find_max_par(input.to_string());
        println!("input: {input:20} expected: {expected} real: {real}",);
        assert_eq!(&real, expected);
    }
}
