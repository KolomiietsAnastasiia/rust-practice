fn rotate(s: String, mut shift: isize) -> String {
    let len = s.len() as isize;
    if len == 0 {
        return s.to_string()
    }

    while shift >= len {
        shift -= len;
    }
    while shift < 0 {
        shift += len;
    }

    let split_index = len as usize - shift as usize;
    let (left, right) = s.split_at(split_index);
    format!("{}{}", right, left)
}

#[test]
fn test() {
    let s = "abcdefgh";
    let shifts = [
        (0,  "abcdefgh"),
        (8,  "abcdefgh"),
        (-8, "abcdefgh"),
        (1,  "habcdefg"),
        (2,  "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10,"cdefghab"),
    ];

    shifts
        .iter()
        .for_each(|(n, exp)|
            assert_eq!(
                rotate(s.to_string(), *n),
                exp.to_string()
            )
        );
}
