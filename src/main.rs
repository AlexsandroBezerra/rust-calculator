use std::io;

const PLUS: char = '+';
const MINUS: char = '-';
const TIMES: char = '*';
const DIVISION: char = '/';
const OPERATIONS: [char; 4] = [PLUS, MINUS, TIMES, DIVISION];

fn main() {
    let n1 = get_float("Digite um número: ");
    let operation = get_operation("Digite uma operação matemática: ");
    let n2 = get_float("Digite outro número: ");
    let result: f32;

    if operation == PLUS {
        result = n1 + n2;
    } else if operation == MINUS {
        result = n1 - n2;
    } else if operation == TIMES {
        result = n1 * n2;
    } else {
        result = n1 / n2;
    }

    println!("{n1} {operation} {n2} = {result}");
}

fn get_float(message: &str) -> f32 {
    loop {
        let mut buf = String::new();
        print!("{message}");
        flush_console();
        io::stdin().read_line(&mut buf).expect("Error reading console!");
        let result = buf.trim().parse();

        if result.is_ok() {
            return result.unwrap();
        } else {
            print!("Número inválido! ");
        };
    };
}

fn get_operation(message: &str) -> char {
    loop {
        let mut buf = String::new();
        print!("{message}");
        flush_console();
        io::stdin().read_line(&mut buf).expect("Error reading console!");
        buf = buf.trim().to_string();

        if buf.len() == 1 {
            let operation = buf.chars().collect::<Vec<char>>()[0];

            if OPERATIONS.contains(&operation) {
                return operation;
            }
        }
    }
}

fn flush_console() {
    io::Write::flush(&mut io::stdout()).expect("Flush failed!");
}