use grammarsmith::*;

#[test]
fn tests() {
    assert_eq!(run("123"), Ok(123));
    assert_eq!(run("123 + 456"), Ok(579));
    assert_eq!(run("123 + 456 * 789"), Ok(359907));
    assert_eq!(run("123 + 456 * 789 / 2"), Ok(180015));
    assert_eq!(run("123 + 456 * 789 / 2 + 3"), Ok(180018));
    assert_eq!(run("123 + 456 * 789 / 2 + 3 * 4"), Ok(180027));
    assert_eq!(run("123 + 456 * 789 / 2 + 3 * 4 / 5"), Ok(180017));
}

fn run(source: &str) -> Result<u64, String> {
    let ast = expr(source);
    ast.eval()
}

fn expr(source: &str) -> CalculatorAST {
    let eof = WithSpan::empty(CalculatorToken::eof());
    let tokens = scan(source);
    let mut parser = Parser::new(&tokens, &eof);
    parse(&mut parser, 0)
}

// Implementation of a simple calculator parser using grammarsmith

fn scan(source: &str) -> Vec<WithSpan<CalculatorToken>> {
    let mut scanner = Scanner::new(source);
    let mut tokens = Vec::new();
    while let Some(c) = scanner.next() {
        if let Some(token) = scan_token(&mut scanner, c) {
            tokens.push(scanner.with_span(token));
        }
        scanner.shift();
    }
    tokens
}

fn scan_token(scanner: &mut Scanner<'_>, c: char) -> Option<CalculatorToken> {
    match c {
        '0'..='9' => {
            scanner.consume_while(|c| c.is_ascii_digit());
            let number = scanner.slice();
            Some(CalculatorToken::Number(number.parse().unwrap()))
        }
        '+' => Some(CalculatorToken::Plus),
        '-' => Some(CalculatorToken::Minus),
        '*' => Some(CalculatorToken::Asterisk),
        '/' => Some(CalculatorToken::Slash),
        _ => None,
    }
}

// see: https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html
fn parse(parser: &mut Parser<'_, CalculatorToken>, min_bp: u8) -> CalculatorAST {
    let mut lhs = match parser.advance().value.clone() {
        CalculatorToken::Number(number) => CalculatorAST::Number(number),
        _ => return CalculatorAST::Error("Expected number".to_string()),
    };

    loop {
        let operator = match parser.peek_token().value.clone() {
            op @ (CalculatorToken::Plus
            | CalculatorToken::Minus
            | CalculatorToken::Asterisk
            | CalculatorToken::Slash) => op,
            CalculatorToken::EOF => return lhs,
            _ => return CalculatorAST::Error("Expected operator".to_string()),
        };

        let (lhs_bp, rhs_bp) = infix_binding_power(&operator);

        if lhs_bp < min_bp {
            break;
        }

        parser.advance();

        let rhs = parse(parser, rhs_bp);

        lhs = match operator {
            CalculatorToken::Plus => CalculatorAST::BinaryOp(
                Box::new(lhs),
                CalculatorBinaryOperator::Plus,
                Box::new(rhs),
            ),
            CalculatorToken::Minus => CalculatorAST::BinaryOp(
                Box::new(lhs),
                CalculatorBinaryOperator::Minus,
                Box::new(rhs),
            ),
            CalculatorToken::Asterisk => CalculatorAST::BinaryOp(
                Box::new(lhs),
                CalculatorBinaryOperator::Multiply,
                Box::new(rhs),
            ),
            CalculatorToken::Slash => CalculatorAST::BinaryOp(
                Box::new(lhs),
                CalculatorBinaryOperator::Divide,
                Box::new(rhs),
            ),
            _ => panic!("Unexpected operator: {:?}", operator),
        }
    }
    lhs
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum CalculatorToken {
    Number(u64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    EOF,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum CalculatorTokenKind {
    Number,
    Plus,
    Minus,
    Asterisk,
    Slash,
    EOF,
}

impl CalculatorToken {
    fn to_kind(&self) -> CalculatorTokenKind {
        match self {
            CalculatorToken::Number(_) => CalculatorTokenKind::Number,
            CalculatorToken::Plus => CalculatorTokenKind::Plus,
            CalculatorToken::Minus => CalculatorTokenKind::Minus,
            CalculatorToken::Asterisk => CalculatorTokenKind::Asterisk,
            CalculatorToken::Slash => CalculatorTokenKind::Slash,
            CalculatorToken::EOF => CalculatorTokenKind::EOF,
        }
    }
}

impl Token for CalculatorToken {
    type Kind = CalculatorTokenKind;

    fn to_kind(&self) -> Self::Kind {
        self.to_kind()
    }
}

impl EndOfFile for CalculatorToken {
    fn eof() -> Self {
        CalculatorToken::EOF
    }

    fn eof_kind() -> Self::Kind {
        CalculatorTokenKind::EOF
    }
}

impl CalculatorToken {
    fn is_operator(&self) -> bool {
        matches!(
            self,
            CalculatorToken::Plus
                | CalculatorToken::Minus
                | CalculatorToken::Asterisk
                | CalculatorToken::Slash
        )
    }
}

enum CalculatorAST {
    Number(u64),
    BinaryOp(
        Box<CalculatorAST>,
        CalculatorBinaryOperator,
        Box<CalculatorAST>,
    ),
    Parenthesized(Box<CalculatorAST>),
    Error(String),
}

impl CalculatorAST {
    pub fn eval(&self) -> Result<u64, String> {
        match self {
            CalculatorAST::Number(n) => Ok(*n),
            CalculatorAST::BinaryOp(lhs, op, rhs) => {
                let lhs_val = lhs.eval()?;
                let rhs_val = rhs.eval()?;
                match op {
                    CalculatorBinaryOperator::Plus => Ok(lhs_val + rhs_val),
                    CalculatorBinaryOperator::Minus => Ok(lhs_val - rhs_val),
                    CalculatorBinaryOperator::Multiply => Ok(lhs_val * rhs_val),
                    CalculatorBinaryOperator::Divide => {
                        if rhs_val == 0 {
                            Err("Division by zero".to_string())
                        } else {
                            Ok(lhs_val / rhs_val)
                        }
                    }
                }
            }
            CalculatorAST::Parenthesized(inner) => inner.eval(),
            CalculatorAST::Error(e) => Err(e.clone()),
        }
    }
}

enum CalculatorBinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

fn infix_binding_power(op: &CalculatorToken) -> (u8, u8) {
    match op {
        CalculatorToken::Plus | CalculatorToken::Minus => (1, 2),
        CalculatorToken::Asterisk | CalculatorToken::Slash => (3, 4),
        _ => (0, 0),
    }
}
