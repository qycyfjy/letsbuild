mod intepreter;
mod token;
mod lexer;
use intepreter::Intepreter;

use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    loop {
        print!("calc> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        if line.contains('q') {
            break;
        }
        let mut i = Intepreter::new(line);
        println!("{}", i.expr());
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn intepret(text: String) -> i32 {
        let mut i = Intepreter::new(text);
        return i.expr();
    }

    #[test]
    fn test_single_digit_ints_add() {
        assert_eq!(intepret("1+1".to_string()), 2);
        assert_eq!(intepret("0+0".to_string()), 0);
        assert_eq!(intepret("9+9".to_string()), 18);
    }

    #[test]
    fn test_multi_digits_ints_add() {
        assert_eq!(intepret("11+1".to_string()), 12);
        assert_eq!(intepret("1010101+2222".to_string()), 1012323);
        assert_eq!(intepret("99+100".to_string()), 199);
    }

    #[test]
    fn test_add_with_spaces() {
        assert_eq!(intepret("   11 +   1".to_string()), 12);
        assert_eq!(intepret("1010101  + 2222".to_string()), 1012323);
        assert_eq!(intepret("99 + 100".to_string()), 199);
    }

    #[test]
    fn test_minus() {
        assert_eq!(intepret("   11 -   1".to_string()), 10);
        assert_eq!(intepret("1012222-2222".to_string()), 1010000);
        assert_eq!(intepret("99 - 100".to_string()), -1);
    }

    #[test]
    fn test_only_add_minus() {
        assert_eq!(intepret("1 + 2 - 3 - 4".to_string()), -4);
        assert_eq!(intepret("2 - 2 + 3 - 3".to_string()), 0);
        assert_eq!(intepret("1 + 1 + 1 + 1 + 1".to_string()), 5);
        assert_eq!(intepret("9+9+9-9-9".to_string()), 9);
    }

    #[test]
    fn test_only_multiply_divide() {
        assert_eq!(intepret("1 * 2 * 3 / 4".to_string()), 1);
        assert_eq!(intepret("2 / 2 * 3 / 3".to_string()), 1);
        assert_eq!(intepret("1 / 1 / 1 / 1 / 1".to_string()), 1);
        assert_eq!(intepret("9*9*9*9/9".to_string()), 729);
    }
}
