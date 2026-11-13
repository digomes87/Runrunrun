#![allow(dead_code)]

use anyhow::{Context, Result};
use rand::Rng;
use rand::{RngExt, distr::Alphanumeric, distr::Uniform};
use rand_distr::Distribution;
use std::fs;

fn main() {
    variables();
    constants();
    read_file().unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        std::process::exit(1);
    });
    formatting_strings();
    random_numbers();
    random_structs();
    generating_password();
    generate_password_user_defined();
    vector_of_integer();
    vector_sort();
}

// ==================== Vector of Integers ====================
fn vector_of_integer() {
    let mut vec = vec![1, 2, 3, 4, 5, 6, 66];
    vec.sort();
    assert_eq!(vec, vec![1, 2, 5, 10, 15]);
}

// ==================== Variables ====================

//
// ==================== Vector Sort ====================
fn vector_sort() {
    let mut vec = vec![1.1_f64, 1.15, 5.5, 1.123, 2.0];
    vec.sort_by(|a, b| a.total_cmp(b));
    assert_eq!(vec, vec![1.1, 1.123, 1.15, 2.0, 5.5]);
}

fn variables() {
    let mut num_10: i32 = 10;
    let num_y = 9;
    let floating_point: f64 = 3.1415333;

    println!("Var: {}, var {}, and var {}", num_10, num_y, floating_point);

    // mutability: only num_10 can be reassigned
    num_10 = 2;

    // shadowing: num_y becomes a NEW variable of type &str
    let num_y = "ABC";

    let mut student_name: &str = "Antonia";
    println!("Student name: {student_name}");
    student_name = "Fagundes";
    println!("New student: {student_name}");

    println!("Now num 10 is: {} and num_y: {}", num_10, num_y);

    // shadowing inside a scope: only exists between the braces
    {
        let num_10 = "number 10";
        println!("inside a scope {num_10}");
    }
}

// ==================== Constants ====================

fn constants() {
    const ATTEMPTS: i32 = 4;
    println!("Amount: {ATTEMPTS}");
}

// ==================== File reading ====================

fn read_file() -> Result<()> {
    let contents = fs::read_to_string("src/file.txt").context("Failed to read src/file.txt")?;
    println!("{contents}");
    Ok(())
}

// ==================== String formatting ====================

fn formatting_strings() {
    let intro = "An introduction again ???";
    let course = "AWS";
    println!("Course {course}, {intro} could be a good one");
}

// ==================== Random numbers ====================

fn random_numbers() {
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

fn random_structs() {
    let mut rng = rand::rng();
    let rand_tuple = rng.random::<(i32, bool, f64)>();
    let rand_point = Point::random(&mut rng);
    println!("Random tuple: {:?}", rand_tuple);
    println!("Random Point: {:?}", rand_point);
}

// ==================== Password generator ====================

fn generating_password() {
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
fn generate_password_user_defined() {
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
