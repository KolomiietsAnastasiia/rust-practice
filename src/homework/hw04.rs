fn rhombus(W: i32) {
    let W_HALF: i32 = W / 2;

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

#[test]
fn test_it() {
    rhombus(11);
    rhombus(3);
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
//
// W = 3;
//  *
// ***
//  *