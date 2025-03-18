#[test]
fn envelope() {
    const W: i32 = 25;
    const H: i32 = 10;

    for y in 0..H {
        let step = y * W/(H - 1);

        for x in 0..W {
            let is_hor = y == 0 || y == H - 1;
            let is_ver = x == 0 || x == W - 1;
            let is_diag = x == step;
            let is_diag2 = x == W - 1 - step;
            let to_show = is_hor || is_ver || is_diag || is_diag2;

            let sym= if to_show { '*' } else { ' ' };
            print!("{}", sym);

        }
        println!();
    }
}

//OUTPUT
// W = 25; H = 10;                  W = 10; H = 10;
// *************************        **********
// * *                   * *        **      **
// *    *             *    *        * *    * *
// *       *       *       *        *  *  *  *
// *          * *          *        *   **   *
// *          * *          *        *   **   *
// *       *       *       *        *  *  *  *
// *    *             *    *        * *    * *
// * *                   * *        **      **
// *************************        **********