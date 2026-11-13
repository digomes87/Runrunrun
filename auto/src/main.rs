#![allow(dead_code)]

use anyhow::{Context, Result};
use rand::Rng;
use rand::{RngExt, distr::Alphanumeric, distr::Uniform};
use rand_distr::Distribution;
use std::fs;

fn main() {
    variaveis();
    constantes();
    leitura_file().unwrap_or_else(|e| {
        eprintln!("Erro: {e}");
        std::process::exit(1);
    });
    formatando_strings();
    numeros_aleatorios();
    structs_aleatorias();
    gerando_senha();
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

// ==================== Variáveis ====================

//
// ==================== Vector Sort ====================
fn vector_sort() {
    let mut vec = vec![1.1_f64, 1.15, 5.5, 1.123, 2.0];
    vec.sort_by(|a, b| a.total_cmp(b));
    assert_eq!(vec, vec![1.1, 1.123, 1.15, 2.0, 5.5]);
}

fn variaveis() {
    let mut num_10: i32 = 10;
    let num_y = 9;
    let ponto_flutuante: f64 = 3.1415333;

    println!("Var: {}, var {}, e var {}", num_10, num_y, ponto_flutuante);

    // mutabilidade: só num_10 pode ser reatribuído
    num_10 = 2;

    // shadowing: num_y vira uma NOVA variável do tipo &str
    let num_y = "ABC";

    let mut nome_aluno: &str = "Antonia";
    println!("Nome da aluna {nome_aluno}");
    nome_aluno = "Fagundes";
    println!("Novo aluno {nome_aluno}");

    println!("Agora o num 10 é: {} e num_y: {}", num_10, num_y);

    // shadowing dentro de um escopo: só existe entre as chaves
    {
        let num_10 = "numero 10";
        println!("dentro de um escopo {num_10}");
    }
}

// ==================== Constantes ====================

fn constantes() {
    const TENTATIVAS: i32 = 4;
    println!("Quantidades: {TENTATIVAS}");
}

// ==================== Leitura de arquivos ====================

fn leitura_file() -> Result<()> {
    let conteudo = fs::read_to_string("src/file.txt").context("Falha ao ler src/file.txt")?;
    println!("{conteudo}");
    Ok(())
}

// ==================== Formatação de strings ====================

fn formatando_strings() {
    let introducao = "Uma introducao de novo ???";
    let curso = "AWS";
    println!("Curso {curso}, {introducao} pode ser uma boa");
}

// ==================== Números aleatórios ====================

fn numeros_aleatorios() {
    // aleatório "puro"
    let mut rng = rand::rng();
    let random_number: u32 = rng.random();
    println!("Random Number: {random_number}");

    // com distribuição (dado de 6 faces até tirar 6)
    let die = Uniform::new_inclusive(1, 6).expect("Falha ao criar distribuição");
    loop {
        let throw = die.sample(&mut rng);
        println!("Roll the die: {throw}");
        if throw == 6 {
            break;
        }
    }
}

// ==================== Structs com random custom ====================

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

fn structs_aleatorias() {
    let mut rng = rand::rng();
    let rand_tuple = rng.random::<(i32, bool, f64)>();
    let rand_point = Point::random(&mut rng);
    println!("Random tupla: {:?}", rand_tuple);
    println!("Random Point: {:?}", rand_point);
}

// ==================== Gerador de senha ====================

fn gerando_senha() {
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
// ==================== Gerador de senha em formato Definido ====================
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
