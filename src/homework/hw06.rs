use rand::Rng;

const PINE: &str = "🟩";
const BACKGROUND: &str = "⬛";
const CONE: &str = "🟫";

fn green() { // генерація шишок
    let mut rnd = rand::rng();
    if rnd.random_range(0..100) <= 10 {
        print!("{}", CONE);
    } else {
        print!("{}", PINE);
    }
}

fn sky() { // небо
    print!("{}", BACKGROUND)
}

fn tree(segments: i32) {
    for segment in 0..segments {
        let space = segments - segment;
        let mut max_levels = 3 + segment;

        for level in 1..max_levels {
            let spaces = space + max_levels - level;
            let greens = level * 2 - 1;
            let line = spaces * 2 + greens;

            for i in 0..line {
                if i >= spaces && i < spaces + greens {
                    green();
                } else {
                    sky();
                }
            }
            println!();
        }
    }
}

#[test]
fn test() {
    tree(5);
}

// segments:5
// OUTPUT:
// ⬛⬛⬛⬛⬛⬛⬛🟩⬛⬛⬛⬛⬛⬛⬛
// ⬛⬛⬛⬛⬛⬛🟩🟩🟩⬛⬛⬛⬛⬛⬛
// ⬛⬛⬛⬛⬛⬛⬛🟩⬛⬛⬛⬛⬛⬛⬛
// ⬛⬛⬛⬛⬛⬛🟩🟩🟩⬛⬛⬛⬛⬛⬛
// ⬛⬛⬛⬛⬛🟩🟩🟩🟩🟩⬛⬛⬛⬛⬛
// ⬛⬛⬛⬛⬛⬛⬛🟩⬛⬛⬛⬛⬛⬛⬛
// ⬛⬛⬛⬛⬛⬛🟩🟫🟩⬛⬛⬛⬛⬛⬛
// ⬛⬛⬛⬛⬛🟩🟩🟩🟫🟩⬛⬛⬛⬛⬛
// ⬛⬛⬛⬛🟩🟩🟩🟩🟩🟩🟩⬛⬛⬛⬛
// ⬛⬛⬛⬛⬛⬛⬛🟩⬛⬛⬛⬛⬛⬛⬛
// ⬛⬛⬛⬛⬛⬛🟩🟩🟩⬛⬛⬛⬛⬛⬛
// ⬛⬛⬛⬛⬛🟫🟫🟩🟩🟩⬛⬛⬛⬛⬛
// ⬛⬛⬛⬛🟩🟩🟫🟩🟩🟩🟫⬛⬛⬛⬛
// ⬛⬛⬛🟩🟩🟩🟩🟩🟩🟩🟩🟩⬛⬛⬛
// ⬛⬛⬛⬛⬛⬛⬛🟩⬛⬛⬛⬛⬛⬛⬛
// ⬛⬛⬛⬛⬛⬛🟩🟩🟩⬛⬛⬛⬛⬛⬛
// ⬛⬛⬛⬛⬛🟩🟩🟩🟩🟩⬛⬛⬛⬛⬛
// ⬛⬛⬛⬛🟩🟩🟩🟩🟩🟩🟫⬛⬛⬛⬛
// ⬛⬛⬛🟩🟫🟩🟩🟩🟩🟩🟩🟩⬛⬛⬛
// ⬛⬛🟩🟩🟩🟩🟩🟩🟫🟩🟩🟩🟩⬛⬛
