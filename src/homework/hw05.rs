fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let num = b;
        b = a % b;
        a = num;
    }
    a
}

#[test]
fn test() {
    let data =
        [
            ((24,  60), 12),
            ((15,   9),  3),
            ((15,   6),  3),
            ((140, 40), 20),
            ((24,  16),  8),
            ((100, 10), 10),
            ((120, 80), 40),
            ((80, 120), 40),
            ((100, 20), 20),
            ((37,  11),  1),
            ((120, 90), 30),
        ];

    for ((a, b), exp) in data.iter() {
        let result = gcd(*a, *b);
        println!("A: {a:3}, B: {b:3}, Expected: {exp:3}, Got: {result}");
        assert_eq!(*exp, result);
    }
}

// OUTPUT:
// A:  24, B:  60, Expected:  12, Got: 12
// A:  15, B:   9, Expected:   3, Got: 3
// A:  15, B:   6, Expected:   3, Got: 3
// A: 140, B:  40, Expected:  20, Got: 20
// A:  24, B:  16, Expected:   8, Got: 8
// A: 100, B:  10, Expected:  10, Got: 10
// A: 120, B:  80, Expected:  40, Got: 40
// A:  80, B: 120, Expected:  40, Got: 40
// A: 100, B:  20, Expected:  20, Got: 20
// A:  37, B:  11, Expected:   1, Got: 1
// A: 120, B:  90, Expected:  30, Got: 30
