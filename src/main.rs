use matrix_det_parser::read_matrix;

fn print_help() {
    println!("Використання:");
    println!("  matrix-det-parser help         Вивести додаткову інформацію");
    println!("  matrix-det-parser credits      Вивести дані");
}

fn print_credits() {
    println!("Обчислення детермінанту матриці");
    println!("Автор: KarachunViktoriia");
    println!("Версія: 0.1.0");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        let matrix = read_matrix();
        let determinant = matrix.determinant();
        println!("Детермінант матриці: {}", determinant);
    } else {
        match args[1].as_str() {
            "help" => print_help(),
            "credits" => print_credits(),
            _ => {
                eprintln!("Невідома команда. Використовуйте 'help' для допомоги.");
                std::process::exit(1);
            }
        }
    }
}
