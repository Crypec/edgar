#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Token {
    Op(Op),
    Num(isize),
    LParen,
    RParen,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Op {
    Plus,
    Minus,
    Multiply,
    Divide,
}

impl Op {
    pub fn presedence(self) -> u8 {
        match self {
            Self::Plus => 1,
            Self::Minus => 1,
            Self::Multiply => 2,
            Self::Divide => 2,
        }
    }
}

pub fn tokenize_str<S: Into<String>>(input: S) -> Vec<Token> {
    let chars = input.into().chars().collect::<Vec<char>>();

    let mut cursor = 0;
    let mut tokens = vec![];

    while let Some(c) = chars.get(cursor) {
        match c {
            c if c.is_whitespace() => (), // skip whitespace
            '+' => tokens.push(Token::Op(Op::Plus)),
            '-' => tokens.push(Token::Op(Op::Minus)),
            '*' => tokens.push(Token::Op(Op::Multiply)),
            '/' => tokens.push(Token::Op(Op::Divide)),
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            c if c.is_digit(10) => {
                let mut num_str = String::from(*c);
                while let Some(c) = chars.get(cursor + 1) {
                    if !c.is_digit(10) {
                        break;
                    }
                    num_str.push(*c);
                    cursor += 1;
                }
                let n = str::parse::<isize>(&num_str).expect("failed to parse number");
                tokens.push(Token::Num(n));
            }
            c => panic!("invalid token: {} :: only integer numbers and the 4 basic math operations are allowed", c),
        };
        cursor += 1;
    }
    tokens
}

pub fn parse_shunting_yard(tokens: Vec<Token>) -> RPNStack {
    let mut output = vec![];
    let mut ops = vec![];
    for t in tokens {
        match t {
            Token::Num(_) => output.push(t),
            Token::Op(current_op) => {
                while let Some(Token::Op(top_op)) = ops.last() {
                    if top_op.presedence() < current_op.presedence() {
                        break;
                    }
                    output.push(ops.pop().unwrap());
                }
                ops.push(t);
            }
            Token::LParen => ops.push(t),
            Token::RParen => {
                while let Some(t) = ops.pop() {
                    if t == Token::LParen {
                        break;
                    }
                    output.push(t);
                }
            }
        }
    }
    output.extend(ops.drain(..).rev());
    output
}

pub type RPNStack = Vec<Token>;

pub fn evaluate_shunting_yard(rpn: RPNStack) -> isize {
    let mut stack = vec![];
    for t in rpn {
        match t {
            Token::Num(n) => stack.push(n),
            Token::Op(op) => {
                let a = stack.pop().expect("misformed rpn :: expected number");
                let b = stack.pop().expect("misformed rpn :: expected number");
                let res = match op {
                    Op::Plus => a + b,
                    Op::Minus => a - b,
                    Op::Multiply => a * b,
                    Op::Divide => a / b,
                };
                stack.push(res);
            }
            _ => panic!("parser bug :: invalid token: {:?}", t),
        };
    }
    stack
        .pop()
        .expect("evaluator bug :: failed to retrieve result from top of stack")
}

fn main() {
    let input = "(42 * 42) + 3 * 2 * 20 + 3";

    let tokens = tokenize_str(input);
    let rpn = parse_shunting_yard(tokens);
    let res = evaluate_shunting_yard(rpn);

    println!("{} = {}", input, res);
}
