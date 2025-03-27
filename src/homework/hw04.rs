#[test]
fn rhombus() {
    const W: i32 = 11;
    const W_HALF: i32 = W / 2;

    for y in 0..W {
        let step = (W_HALF - y).abs();

        for x in 0..W {
            let left = x >= step;
            let right = x < W - step;
            let to_show = left & right;

            let sym= if to_show { '*' } else { ' ' };
            print!("{}", sym);

        }
        println!();
    }
}

//OUTPUT
// W = 11;
//      *
//     ***
//    *****
//   *******
//  *********
// ***********
//  *********
//   *******
//    *****
//     ***
//      *