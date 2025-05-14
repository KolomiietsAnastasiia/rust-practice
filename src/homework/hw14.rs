fn gray(n: u8) -> Vec<String> {
    let mut result = Vec::new();
    let mut total = 1;

    if n == 0 {
        result.push("".to_string());
        return result;
    }

    for _ in 0..n {
        total *= 2;
    }

    for num in 0..total {
        let mut s = String::new();
        let mut x = num;

        for _ in 0..n {
            if x % 2 == 0 {s = "0".to_string() + &s}
            else {s = "1".to_string() + &s}

            x = x / 2;
        }

        result.push(s);
    }

    result
}

#[test]
fn test() {
    let test_data =
        [
            (0, vec!("")),
            (1, vec!("0", "1")),
            (2, vec!("00", "01", "10", "11")),
            (3, vec!("000", "001", "010", "011",
                     "100", "101", "110", "111")),
            (4, vec!("0000", "0001", "0010", "0011",
                     "0100", "0101", "0110", "0111",
                     "1000", "1001", "1010", "1011",
                     "1100", "1101", "1110", "1111")),
        ];


    test_data
        .iter()
        .for_each(|(n, out)|
            assert_eq!(gray(*n), *out)
        );
}