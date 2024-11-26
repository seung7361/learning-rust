use rand::Rng;

pub fn random_i32(min: i32, max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

pub fn random_usize(min: usize, max: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

pub fn two_digit(x: usize) -> String {
    let symbol = match x {
        0..=9 => format!(" {x}"),
        _ => format!("{x}"),
    };

    symbol
}