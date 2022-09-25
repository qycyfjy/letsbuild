mod intepreter;
mod token;
mod lexer;
mod ast;
mod parser;
mod visitor;

use std::io::{self, Write};

use crate::{visitor::{PostOrderVisitor}};

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
        let mut p = parser::Parser::new(line);
        println!("{:#?}", p.parse());
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn eval(text: String) -> i32 {
        let mut i = intepreter::Intepreter::new(text, PostOrderVisitor{});
        i.eval()
    }

    #[test]
    fn test_single_digit_ints_add() {
        assert_eq!(eval("1+1".to_string()), 2);
        assert_eq!(eval("0+0".to_string()), 0);
        assert_eq!(eval("9+9".to_string()), 18);
    }

    #[test]
    fn test_multi_digits_ints_add() {
        assert_eq!(eval("11+1".to_string()), 12);
        assert_eq!(eval("1010101+2222".to_string()), 1012323);
        assert_eq!(eval("99+100".to_string()), 199);
    }

    #[test]
    fn test_add_with_spaces() {
        assert_eq!(eval("   11 +   1".to_string()), 12);
        assert_eq!(eval("1010101  + 2222".to_string()), 1012323);
        assert_eq!(eval("99 + 100".to_string()), 199);
    }

    #[test]
    fn test_minus() {
        assert_eq!(eval("   11 -   1".to_string()), 10);
        assert_eq!(eval("1012222-2222".to_string()), 1010000);
        assert_eq!(eval("99 - 100".to_string()), -1);
    }

    #[test]
    fn test_only_add_minus() {
        assert_eq!(eval("1 + 2 - 3 - 4".to_string()), -4);
        assert_eq!(eval("2 - 2 + 3 - 3".to_string()), 0);
        assert_eq!(eval("1 + 1 + 1 + 1 + 1".to_string()), 5);
        assert_eq!(eval("9+9+9-9-9".to_string()), 9);
    }

    #[test]
    fn test_only_multiply_divide() {
        assert_eq!(eval("1 * 2 * 3 / 4".to_string()), 1);
        assert_eq!(eval("2 / 2 * 3 / 3".to_string()), 1);
        assert_eq!(eval("1 / 1 / 1 / 1 / 1".to_string()), 1);
        assert_eq!(eval("9*9*9*9/9".to_string()), 729);
    }

    #[test]
    fn test_basic_arithmetic() {
        assert_eq!(eval("1 + 2 * 3 - 4".to_string()), 3);
        assert_eq!(eval("2 / 2 + 3 * 3".to_string()), 10);
        assert_eq!(eval("22 + 30 - 10 * 4/4".to_string()), 42);
        assert_eq!(eval("9+9*9-9/9".to_string()), 89);
    }

    #[test]
    fn test_parentheses_arithmetic() {
        assert_eq!(eval("(1 + 2) * 3 - 4".to_string()), 5);
        assert_eq!(eval("2 / 2 + 3 * 3".to_string()), 10);
        assert_eq!(eval("22 + 30 - 10 * 4/4".to_string()), 42);
        assert_eq!(eval("9+9*(9-9/9)".to_string()), 81);
        assert_eq!(eval("(1+1*1*10/(1+3)) * (2-(3+2))".to_string()), -9);
    }

    #[test]
    fn test_unary() {
        assert_eq!(eval("-1".to_string()), -1);
        assert_eq!(eval("-1+1".to_string()), 0);
        assert_eq!(eval("-(1+1)".to_string()), -2);
        assert_eq!(eval("1--1".to_string()), 2);
        assert_eq!(eval("+1--2".to_string()), 3);
        assert_eq!(eval("+1*-2".to_string()), -2);
    }
}
