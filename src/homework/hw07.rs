fn invert_the_case(s: String) -> String {
    let mut new = String::new();

    for letter in s.chars() {
        new.push_str(
            &if letter.is_lowercase() {
                letter.to_uppercase().to_string()
            } else if letter.is_uppercase() {
                letter.to_lowercase().to_string()
            } else {
                letter.to_string()
            }
        );
    }

    new
}

#[test]
fn test() {
    let data =
        [
            ("Hello", "hELLO"),
            ("Привет", "пРИВЕТ"),
        ];

    data
        .iter()
        .for_each(|(a, b)| {
            assert_eq!(
                invert_the_case(a.to_string()),
                b.to_string()
            );
            assert_eq!(
                invert_the_case(b.to_string()),
                a.to_string()
            );
        });
}
