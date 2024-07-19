
//parser.rs
use crate::scanner::{self, Scanner, Token};
use crate::ast::TreeNode;

pub struct Parser<'a>{
    scanner: Scanner<'a>,
    curr_tok: Token,
}

impl<'a> Parser<'a> {
    pub fn new(mut scanner:Scanner<'a>) -> Self {
        let curr_token = scanner.next_token();
        Parser{scanner,curr_tok: curr_token}
    }

    fn advance(&mut self) {
        self.curr_tok = self.scanner.next_token(); 
    }

    fn parse_factor(&mut self) -> TreeNode {
        let current_token = self.curr_tok.clone(); // Clone the current token to avoid borrowing issues
        match current_token {
            Token::Number(n) => {
                self.advance();
                TreeNode::Number(n)
            }
            Token::Identifier(id) => {
                self.advance();
                TreeNode::Identifier(id.clone())
            }
            _ => panic!("unexpected token: {:?}", self.curr_tok),
        }
    }

    fn parse_term(&mut self) -> TreeNode {
        let mut left = self.parse_factor();

        while let Token::Star | Token::Slash = self.curr_tok {
            let op = self.curr_tok.clone();
            self.advance();
            let right = self.parse_factor();
            left = TreeNode::BinaryOp(Box::new(left), op, Box::new(right));
        }

        left
    }

    fn parse_exp(&mut self) -> TreeNode {
        let mut left = self.parse_term();

        while let Token::Plus | Token::Minus = self.curr_tok {
            let op = self.curr_tok.clone();
            self.advance();
            let right = self.parse_term();
            left = TreeNode::BinaryOp(Box::new(left), op, Box::new(right));
        }

        left
    }

    pub fn parse(&mut self) -> TreeNode {
        self.parse_exp()
    }
}