// ==================== Random numbers, structs, and passwords ====================

use rand::Rng;
use rand::{RngExt, distr::Alphanumeric, distr::Uniform};
use rand_distr::Distribution;

pub fn random_numbers() {
    // "pure" random
    let mut rng = rand::rng();
    let random_number: u32 = rng.random();
    println!("Random Number: {random_number}");

    // with a distribution (6-sided die until we roll a 6)
    let die = Uniform::new_inclusive(1, 6).expect("Failed to create distribution");
    loop {
        let throw = die.sample(&mut rng);
        println!("Roll the die: {throw}");
        if throw == 6 {
            break;
        }
    }
}

// ==================== Structs with custom random ====================

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn random<R: Rng>(rng: &mut R) -> Self {
        Point {
            x: rng.random(),
            y: rng.random(),
        }
    }
}

pub fn random_structs() {
    let mut rng = rand::rng();
    let rand_tuple = rng.random::<(i32, bool, f64)>();
    let rand_point = Point::random(&mut rng);
    println!("Random tuple: {:?}", rand_tuple);
    println!("Random Point: {:?}", rand_point);
    println!("Point coordinates: ({}, {})", rand_point.x, rand_point.y);
}

// ==================== Password generator ====================

pub fn generating_password() {
    let password = generate_password();
    println!("Your password {password}");
}

fn generate_password() -> String {
    const PASSWORD_LEN: usize = 30;
    let mut rng = rand::rng();

    (0..PASSWORD_LEN)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect()
}

// ==================== Password generator with a defined charset ====================

pub fn generate_password_user_defined() {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                        abcdefghijklmnopqrstuvwxyz\
                        0123456789)(*&^%$#@!~";
    const PASSWORD_LEN: usize = 30;

    let mut rng = rand::rng();

    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    println!("New password: {password}");
}
