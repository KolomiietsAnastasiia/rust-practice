use std::cmp::{max, min};
use std::panic::panic_any;

fn roof_size0(min: u32, max: u32) -> u32 {
    max - min + 1
}
fn roof_size(n1: u32, n2: u32, n3: u32) -> u32 {
    let mut v = [n1, n2, n3];
    v.sort();
    match v {
        [min,_, max] => roof_size0(min, max),
    }
}
fn roof_size2(n1: u32, n2: u32, n3: u32) -> u32 {
    let mn = min(min(n1, n2), n3);
    let mx = max(max(n1, n2), n3);

    mx - mn + 1
}
fn car_roof_size(cars: &[u32], k: usize) -> u32 {
    let mut min_size = u32::MAX;

    let mut cars = cars.to_vec();
    cars.sort();

    let car = |idx: u32| cars[idx as usize];

    for idx in 0..=(cars.len() as u32 - k as u32) {
        let mut size = roof_size0(car(idx), car(idx + (k-1) as u32));

        if size < min_size {min_size = size;}
    }

    min_size
}

#[test]
fn test1() {
    let cars = [6, 2, 12, 7];
    let k = 3;
    let roof_expected = 6;
    let roof_real = car_roof_size(&cars, k);

    println!("Min: {}", roof_real);
    assert_eq!(roof_expected, roof_real);
}

#[test]
fn size_test() {
    let data = [
        ((1,2,3),3),
        ((1,2,4),4),
        ((10,11,12),3),
        ((100,200,150),101),
    ];

    for ((n1, n2, n3), expected) in data {
        let real = roof_size(n1, n2, n3);
        assert_eq!(roof_size(n1, n2, n3), expected);
    }
}