### matrix-det-parser

**matrix-det-parser** - це інструмент командного рядка для обчислення детермінанту матриці з використанням бібліотеки `nalgebra`.

### Встановлення 

Додайте наступний рядок до секції `[dependencies]` у вашому файлі `Cargo.toml`:

```toml
matrix-det-lib = { version = "0.1.0", path = "C:\\krs\\matrix-det-lib" }
```

### Приклад

```rust
use matrix_det_lib::*;

fn main() {
    let matrix = read_matrix();
    let determinant = matrix.determinant();
    println!("Детермінант матриці: {}", determinant);
}
```

### matrix_input = { "3" ~ "\n" ~ number* }
```
#[test]
fn test_matrix_input() {
    let input = "3\n1.0 2.0 3.0\n4.0 5.0 6.0\n7.0 8.0 9.0\n";
    let result = MatrixParser::parse(Rule::matrix_input, input);
    assert!(result.is_ok());
}
```

### dimension_line = @{ integer ~ "\n" }
```
#[test]
fn test_dimension_line() {
    let input = "3\n";
    let result = MatrixParser::parse(Rule::dimension_line, input);
    assert!(result.is_ok());
}
```

### matrix_lines = { matrix_line+ }
```
#[test]
fn test_matrix_lines() {
    let input = "1.0 2.0 3.0\n4.0 5.0 6.0\n7.0 8.0 9.0\n";
    let result = MatrixParser::parse(Rule::matrix_lines, input);
    assert!(result.is_ok());
}
```

### matrix_line = { number+ ~ "\n" }
```
#[test]
fn test_matrix_line() {
    let input = "1.0 2.0 3.0\n";
    let result = MatrixParser::parse(Rule::matrix_line, input);
    assert!(result.is_ok());
} 
```
