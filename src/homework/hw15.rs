#[test]
fn main() {
    let mut solutions = Vec::new();

    for m in 1..=8 { for u in 1..=8 { for x in 1..=8 { for a in [2, 4, 8].iter() {
    for s in 1..=8 { for l in 1..=8 { for o in 1..=8 { for n in [4, 6, 4].iter() {

        let digits = [m, u, x, *a, s, l, o, *n];
        let mut unique = true;
        for i in 0..8 {
            for j in i+1..8 {
                if digits[i] == digits[j] { unique = false; }
            }
        }
        if !unique { continue; }

        let muha = m * 1000 + u * 100 + x * 10 + a;
        let slon = s * 1000 + l * 100 + o * 10 + n;

        if muha as u32 * (*a as u32) == slon {
            solutions.push((m, u, x, *a, s, l, o, *n, muha, slon));
        }

        }}}}}}}}


    for sol in &solutions {
        println!("  {:4}", sol.8); // МУХА
        println!("х    {}", sol.3); // А
        println!(" -----");
        println!("  {:4}", sol.9); // СЛОН
        println!("Цифри: М={}, У={}, Х={}, А={}, С={}, Л={}, О={}, Н={}",
                 sol.0, sol.1, sol.2, sol.3, sol.4, sol.5, sol.6, sol.7);
        println!();
    }
}

// OUTPUT
//
//   1782
// х    2
//  -----
//   3564
// Цифри: М=1, У=7, Х=8, А=2, С=3, Л=5, О=6, Н=4
//
//   1782
// х    2
//  -----
//   3564
// Цифри: М=1, У=7, Х=8, А=2, С=3, Л=5, О=6, Н=4
//
//   3582
// х    2
//  -----
//   7164
// Цифри: М=3, У=5, Х=8, А=2, С=7, Л=1, О=6, Н=4
//
//   3582
// х    2
//  -----
//   7164
// Цифри: М=3, У=5, Х=8, А=2, С=7, Л=1, О=6, Н=4