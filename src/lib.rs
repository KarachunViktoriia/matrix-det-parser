extern crate pest_derive;
extern crate pest;
#[macro_use]
pub mod grammar;

extern crate nalgebra as na;

use std::io;

pub fn read_matrix() -> na::DMatrix<f64> {
    println!("Введіть розмірність матриці:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Не вдалося прочитати рядок");
    let n: usize = match input.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Некоректне введення для розмірності матриці");
            std::process::exit(1);
        }
    };

    println!("Введіть елементи матриці (розділені пробілами):");

    let matrix_data: Vec<f64> = (0..n)
        .flat_map(|_| {
            let mut row = String::new();
            io::stdin()
                .read_line(&mut row)
                .expect("Не вдалося прочитати рядок");

            row.split_whitespace()
                .filter_map(|s| {
                    let parsed = s.parse().ok();
                    if parsed.is_none() {
                        eprintln!("Помилка парсингу для: {:?}", s);
                    }
                    parsed
                })
                .collect::<Vec<_>>() // Збираємо значення у вектор
        })
        .collect();

    println!("Debug: {:?}", matrix_data); // Доданий вивід для відладки

    na::DMatrix::from_row_slice(n, n, &matrix_data)
}
