struct Point {
    x: i32,
    y: i32,
}

struct Rectangle {
    a: Point,
    b: Point,
}

fn area_occupied(rectangles: &[Rectangle])  -> i32 {
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    for rect in rectangles {
        let x_start = rect.a.x.min(rect.b.x);
        let x_end = rect.a.x.max(rect.b.x);
        let y_start = rect.a.y.min(rect.b.y);
        let y_end = rect.a.y.max(rect.b.y);

        min_x = min_x.min(x_start);
        max_x = max_x.max(x_end);
        min_y = min_y.min(y_start);
        max_y = max_y.max(y_end);
    }

    // Массив для зберігання зайнятих сегментів
    let width = (max_x - min_x) as usize;
    let height = (max_y - min_y) as usize;
    let mut segments_grid = vec![vec![false; width]; height];

    // Заповнення массиву (для кожного прямокутника)
    for rect in rectangles {
        let x_start = rect.a.x.min(rect.b.x) - min_x;
        let x_end = rect.a.x.max(rect.b.x) - min_x;
        let y_start = rect.a.y.min(rect.b.y) - min_y;
        let y_end = rect.a.y.max(rect.b.y) - min_y;

        for x in x_start..x_end {
            for y in y_start..y_end {
                segments_grid[y as usize][x as usize] = true;
            }
        }
    }

    // Кількість зайнятих сегментів
    let mut count = 0;
    for row in segments_grid {
        for cell in row {
            if cell {
                count += 1;
            }
        }
    }

    count
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

#[test]
fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
}