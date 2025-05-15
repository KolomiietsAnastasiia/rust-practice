fn countValleys(s: String) -> i32 {
    let mut count = 0;
    let mut current = 0;

    for c in s.chars() {
        match c {
            'U' => current += 1,
            'D' => {
                if current == 0 { count += 1 };
                current -= 1;
            } ,
            _ => continue,
        }
    }

    count
}

fn countValleys2(s: String) -> i32 {
    let mut count = 0;
    let mut current = 0;

    for c in s.chars() {
        match c {
            'U' => current += 1,
            'D' => current -= 1,
            _ => continue,
        }
        if current == -1 { count += 1 }
    }


    count/2
}

#[test]
fn test() {
    let paths = [
        ("UDDDUDUU", 1),
        ("UUDDDDUUDDDUUU", 2),
    ];

    for (p, n) in paths {
        println!("{:?} -> {}", countValleys2(p.to_string()), n);
    }
}