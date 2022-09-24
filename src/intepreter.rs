// use crate::{token::Token, lexer::Lexer};

// pub struct Intepreter {
//     lexer: Lexer,
// }

// impl Intepreter {
//     pub fn new(text: String) -> Self {
//         let mut i = Intepreter {
//             lexer: Lexer::new(text),
//         };
//         i.lexer.get_next_token();
//         i
//     }
// }

// impl Intepreter {
//     // factor : INTEGER | LParen expr RParen
//     fn factor(&mut self) -> i32 {
//         match self.lexer.get_next_token() {
//             Some(Token::Integer(num)) => {
//                 self.lexer.get_next_token();
//                 num
//             }
//             Some(Token::LParen) => {
//                 self.lexer.get_next_token();
//                 let v = self.expr();
//                 self.lexer.get_next_token();  // Skip RParen
//                 v
//             }
//             _ => panic!("expect a integer")
//         }
//     }

//     // term : factor ((MUL | DIV) factor)*
//     fn term(&mut self) -> i32 {
//         let mut left = self.factor();
//         while let Some(token) = self.lexer.get_next_token() {
//             match token {
//                 Token::Multiply => {
//                     self.lexer.get_next_token();
//                     left *= self.factor()
//                 }
//                 Token::Divide => {
//                     self.lexer.get_next_token();
//                     let right = self.factor();
//                     if right == 0 {
//                         panic!("divide 0");
//                     }
//                     left /= right;
//                 }
//                 _ => break,
//             }
//         }
//         left
//     }

//     // expr : term ((PLUS | SUB) term)*
//     pub fn expr(&mut self) -> i32 {
//         let mut left = self.term();
//         while let Some(token) = self.lexer.get_next_token() {
//             match token {
//                 Token::Plus => {
//                     self.lexer.get_next_token();
//                     left += self.term()
//                 }
//                 Token::Subtract => {
//                     self.lexer.get_next_token();
//                     left -= self.term()
//                 }
//                 _ => break,
//             }
//         }
//         left
//     }
// }

