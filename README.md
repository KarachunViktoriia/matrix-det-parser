# matrix-det-parser

**matrix-det-parser** - це інструмент командного рядка для обчислення детермінанту матриці з використанням бібліотеки `nalgebra`.

## Встановлення

Додайте наступний рядок до секції `[dependencies]` у вашому файлі `Cargo.toml`:

```toml
matrix-det-lib = { version = "0.1.0", path = "C:\\krs\\matrix-det-lib" }

### Приклад

```rust
use matrix_det_lib::*;

fn main() {
    let matrix = read_matrix();
    let determinant = matrix.determinant();
    println!("Детермінант матриці: {}", determinant);
}

// Граматичне правило Number
number: f64 = @{
    ("-")? // Необов'язковий від'ємний знак
    ("0" | ASCII_NONZERO_DIGIT ~ ASCII_DIGIT*) // Ціле чи десяткове число
    ~ ("." ~ ASCII_DIGIT*)? // Опціональна десяткова частина
}

#[test]
fn test_number_parsing() {
    assert_eq!(parse_number("42"), Ok(ast::MatrixExpr::Number(42.0)));
    assert_eq!(parse_number("3.14"), Ok(ast::MatrixExpr::Number(3.14)));
}

// Граматичне правило UnaryMinus
unary_minus: na::DMatrix<f64> = {
    "-" expr
}

#[test]
fn test_unary_minus_parsing() {
    assert_eq!(
        parse_unary_minus("-42"),
        Ok(ast::MatrixExpr::UnaryMinus(Box::new(ast::MatrixExpr::Number(42.0))))
    );
}

// Граматичне правило для Matrix
matrix: ast::MatrixExpr = {
    "[" (expr (~ "," ~ expr)*)? "]"
}

#[test]
fn test_matrix_parsing() {
    assert_eq!(
        parse_matrix_expr("[[1.0, 2.0], [3.0, 4.0]]"),
        Ok(ast::MatrixExpr::Matrix(vec![
            vec![1.0, 2.0],
            vec![3.0, 4.0]
        ]))
    );
}

// Граматичне правило BinaryOp
binary_op: ast::MatrixExpr = {
    expr "+" expr
    | expr "-" expr
    | expr "*" expr
    | expr "/" expr
}

#[test]
fn test_matrix_operations() {
    assert_eq!(
        parse_matrix_expr("[[1.0, 2.0], [3.0, 4.0]] + [[2.0, 1.0], [1.0, 2.0]]"),
        Ok(ast::MatrixExpr::BinaryOp(
            Box::new(ast::MatrixExpr::Matrix(vec![
                vec![1.0, 2.0],
                vec![3.0, 4.0]
            ])),
            ast::MatrixBinOp::Add,
            Box::new(ast::MatrixExpr::Matrix(vec![
                vec![2.0, 1.0],
                vec![1.0, 2.0]
            ]))
        ))
    );
}
