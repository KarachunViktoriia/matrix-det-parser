use matrix_det_parser::grammar::*;
use pest::Parser;
use crate::MatrixParser;

#[test]
fn test_matrix_input() {
    let input = "3\n1.0 2.0 3.0\n4.0 5.0 6.0\n7.0 8.0 9.0\n";
    let result = MatrixParser::parse(Rule::matrix_input, input);
    assert!(result.is_ok());
}

#[test]
fn test_dimension_line() {
    let input = "3\n";
    let result = MatrixParser::parse(Rule::dimension_line, input);
    assert!(result.is_ok());
}

#[test]
fn test_matrix_lines() {
    let input = "1.0 2.0 3.0\n4.0 5.0 6.0\n7.0 8.0 9.0\n";
    let result = MatrixParser::parse(Rule::matrix_lines, input);
    assert!(result.is_ok());
}

#[test]
fn test_matrix_line() {
    let input = "1.0 2.0 3.0\n";
    let result = MatrixParser::parse(Rule::matrix_line, input);
    assert!(result.is_ok());
}

