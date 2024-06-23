//! # Parsing module
//! This contains all functions and structs needed to parse source code into
//! an Abstract Syntax Tree.
use std::ptr::read;
use log::{debug, error, info};

/// Expressions return a value.
pub enum Expression {
    Value {
        value:Token,
    },
    AdditionExp {
        left: Box<Expression>,
        right: Box<Expression>,
    }
}
pub enum Statement {
    LetStatement {
        left: Token,
        right: Expression,
    },
    IfStatement {
        condition: Expression,
        true_path: Box<Statement>,
    },
    ReturnStatement { 
        return_output: Expression,
    },
    BlockStatement {
        statements: Vec<Box<Statement>>,
    }
}




/// This parses plaintext (source code) to an abstract syntax tree by first tokenising it,
/// then parsing the tokens to an AST.
pub fn text_to_ast(input: String) -> Vec<Statement> {
    let tokens: Vec<Token> = tokenise(input.chars().collect());
    info!("Successfully tokenised input");
    let program: Vec<Statement> = token_to_ast(tokens);
    return program;
}

// TODO: CONTINUE HERE.
pub fn token_to_ast(tokens: Vec<Token>) -> Vec<Statement> {
    let mut program: Vec<Statement> = vec![];
    let mut read: usize = 0;
    while read < tokens.len() {
        let node: Statement = parse_statement(&tokens, &read);
        read += 1;
    }
    return program;
}

pub fn parse_statement(tokens: &Vec<Token>, read_pos: &usize) -> Statement {
    return match tokens[*read_pos] {
        Token::LetToken => parse_let_statement(tokens, read_pos),
        _ => todo!(),
    }
}
pub fn parse_let_statement(tokens: &Vec<Token>, read_pos: &usize) -> Statement {
    // Checks to make sure that syntax is valid.
    // TODO: Implement types and type checking.
    if let Some(Token::ID(_)) = peek_token(tokens, read_pos, 1) {
        error!("Expected identifier for let assignment (e.g. let x = 3)");
    }
    if peek_token(tokens, read_pos, 2) != Some(Token::EqComparator) {
        error!("Expected \"=\" for let assignment (e.g. let x = 3)");
    }
    let lhs: Token = tokens[*read_pos+1].clone();
    let rhs: Expression = substring_to_outer_semicolon(&tokens[*read_pos+3..]);
    return Statement::LetStatement { left: lhs, right: rhs};
}

// TODO: This needs to be extended to deal with nested blocks and inner semicolons.
// Maybe convert to return the index of the semicolon.
pub fn substring_to_outer_semicolon(tokens: &[Token]) -> Expression {
    let index: usize = match tokens.iter().position(|m| m == &Token::Semicolon) {
        Some(x) => x,
        None => {
            error!("Expected semicolon at end of line.");
            panic!("Syntax error.");
        }
    };
    return Expression::Value{value:tokens[index].clone()};
}



/// Peeks the next token in the list of tokens if possible.
fn peek_token(tokens: &Vec<Token>, read_pos: &usize, step: usize) -> Option<Token> {
    return if *read_pos + step >= tokens.len() {
        Option::None
    } else {
        Option::Some(tokens[*read_pos + 1].clone())
    }
}


/// This tokenises the whole input by constructing a lexer struct
/// and passing it to `next_token()` for tokenising.
pub fn tokenise(input: Vec<char>) -> Vec<Token> {
    let mut lexer: Lexer = Lexer{
        input,
        read_position: 1,
        position: 0};
    let mut tokens: Vec<Token> = vec![];
    // Tokenise symbolic tokens, as well as identify IDs and Literals.
    while lexer.position < lexer.input.len() {
        if (lexer.input[lexer.position] == ' ') || (lexer.input[lexer.position] == '\r') {
            // If this is space then eat whitespace.
            lexer.position += 1;
            lexer.read_position += 1;
        }
        else {
            let tok: Token = next_token(&mut lexer);
            tokens.push(tok);
        }
    }
    let mut index: usize = 0;
    // Tokenise keywords from existing IDs.
    while index < tokens.len() {
        tokens[index] = pull_keywords(&tokens[index]);
        index += 1;
    }
    return tokens;
}

/// This pulls keywords from IDs.
fn pull_keywords(token: &Token) -> Token {
    return match token {
        Token::ID(id_string)  => {
            return match id_string.as_str() {
                "if" => Token::IfToken,
                "else" => Token::ElseToken,
                "while" => Token::WhileToken,
                "in" => Token::InToken,
                "fn" => Token::FnToken,
                "return" => Token::ReturnToken,
                "let" => Token::LetToken,
                other => Token::ID(other.parse().unwrap()),
            }
        }
        other => other.clone(),
    }
}

/// This Lexer struct is passed around to compute the tokens.
#[derive(Debug, Clone, Eq, PartialEq)]
struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
}

/// This peeks to the next character in the input string using
/// the read_position.
fn peek_char(lexer: &Lexer) -> Option<char> {
    return if lexer.read_position < lexer.input.len() {
        Option::Some(lexer.input[lexer.read_position])  
    } else {
        Option::None
    }
}

/// The following are neyman's current token set.
/// This will likely expanded on once the initial features are established.
///
/// | Token Type      | Expected Literal |
/// | --------------- | ---------------- |
/// | LeftBrace       | {                |
/// | RightBrace      | }                |
/// | LeftBracket     | [                |
/// | RightBracket    | ]                |
/// | LeftParen       | (                |
/// | RightParen      | )                |
/// | Semicolon       | ;                |
/// | Colon           | :                |
/// | Period          | .                |
/// | RightArrow      | ->               |
/// | Comment(String) | *n.a.*           |
/// | LeqComparator   | <=               |
/// | GeqComparator   | >=               |
/// | LeComparator    | <                |
/// | GeComparator    | >                |
/// | EqComparator    | ==               |
/// | NeqComparator   | !=               |
/// | AndOperation    | &&               |
/// | OrOperation     | \|               |
/// | Assign          | =                |
/// | Asterisk        | *                |
/// | Ampersand       | &                |
/// | Dash            | -                |
/// | Slash           | /                |
/// | QuestionMark    | ?                |
/// | Plus            | +                |
/// | Pipe            | \|               |
/// | IfToken         | if               |
/// | ElseToken       | else             |
/// | WhileToken      | while            |
/// | InToken         | in               |
/// | FnToken         | fn               |
/// | ReturnToken     | return           |
/// | LetToken        | let              |
/// | ID(String)      | *n.a.*           |
/// | Literal(String) | *n.a.*           |
/// | EOL             | \r\n *or* \n     |
/// | Illegal         | *n.a.*           |
/// | Underscore      | _                |
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Token {
    // Language punctuation and operators
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    LeftParen,
    RightParen,
    Semicolon,
    Colon,
    Period,
    RightArrow,
    Comment(String),
    LeqComparator,
    GeqComparator,
    LeComparator,
    GeComparator,
    EqComparator,
    NeqComparator,
    AndOperation,
    OrOperation,
    Assign,
    Asterisk,
    Ampersand,
    Dash,
    Slash,
    QuestionMark,
    Plus,
    Pipe,
    ExclamationMark,
    Underscore,

    // Language keywords
    IfToken,
    ElseToken,
    WhileToken,
    InToken,
    FnToken,
    ReturnToken,
    LetToken,

    ID(String),
    Literal(String),

    EOL,
    Illegal,
}

/// This tokenises the next token in the lexer struct using position.
fn next_token(lexer: &mut Lexer) -> Token {
    return match lexer.input[lexer.position] {
        '{' => {
            lexer.position += 1;
            lexer.read_position += 1;
            Token::LeftBrace
        },
        '}' => {
            lexer.position += 1;
            lexer.read_position += 1;
            Token::RightBrace
        }
        '[' => {
            lexer.position += 1;
            lexer.read_position += 1;
            Token::LeftBracket
        }
        ']' => {
            lexer.position += 1;
            lexer.read_position += 1;
            Token::RightBracket
        }
        '(' => {
            lexer.position += 1;
            lexer.read_position += 1;
            Token::LeftParen
        }
        ')' => {
            lexer.position += 1;
            lexer.read_position += 1;
            Token::RightParen
        }
        ';' => {
            lexer.position += 1;
            lexer.read_position += 1;
            Token::Semicolon
        }
        ':' => {
            lexer.position += 1;
            lexer.read_position += 1;
            Token::Colon
        }
        '.' => {
            lexer.position += 1;
            lexer.read_position += 1;
            Token::Period
        }
        '-' => {
            let tok: Token;
            if peek_char(lexer) == Option::Some('>') {
                tok = Token::RightArrow;
                lexer.position += 2;
                lexer.read_position += 2;
            } else {
                tok = Token::Dash;
                lexer.position += 1;
                lexer.read_position += 1;
            }
            return tok;
        }
        '>' => {
            let tok: Token;
            if peek_char(lexer) == Option::Some('=') {
                tok = Token::GeqComparator;
                lexer.position += 2;
                lexer.read_position += 2;
            } else {
                tok = Token::GeComparator;
                lexer.position += 1;
                lexer.read_position += 1;
            }
            return tok;
        }
        '/' => {
            let tok: Token;
            if peek_char(lexer) == Option::Some('/') {
                let Some(index) = lexer.input[lexer.position..]
                    .iter()
                    .position(|&c| c == '\n')
                    else {
                        error!("Could not find newline ending.");
                        panic!("No newline token at end of file for comment.");
                    };
                tok = Token::Comment(lexer.input[lexer.position..index + lexer.position]
                    .iter()
                    .collect()
                );
                lexer.position = index + lexer.position;
                lexer.read_position = lexer.position + 1;

            } else {
                tok = Token::Slash;
                lexer.position += 1;
                lexer.read_position += 1;
            }
            return tok;
        }
        '<' => {
            let tok: Token;
            if peek_char(lexer) == Option::Some('=') {
                tok = Token::LeqComparator;
                lexer.position += 2;
                lexer.read_position += 2;
            } else {
                tok = Token::LeComparator;
                lexer.position += 1;
                lexer.read_position += 1;
            }
            return tok;
        }
        '=' => {
            let tok: Token;
            if peek_char(lexer) == Option::Some('=') {
                tok = Token::EqComparator;
                lexer.position += 2;
                lexer.read_position += 2;
            } else {
                tok = Token::Assign;
                lexer.position += 1;
                lexer.read_position += 1;
            }
            return tok;
        }
        '!' => {
            let tok: Token;
            if peek_char(lexer) == Option::Some('=') {
                tok = Token::NeqComparator;
                lexer.position += 2;
                lexer.read_position += 2;
            } else {
                tok = Token::ExclamationMark;
                lexer.position += 1;
                lexer.read_position += 1;
            }
            return tok;
        }
        '&' => {
            let tok: Token;
            if peek_char(lexer) == Option::Some('&') {
                tok = Token::AndOperation;
                lexer.position += 2;
                lexer.read_position += 2;
            } else {
                tok = Token::Ampersand;
                lexer.position += 1;
                lexer.read_position += 1;
            }
            return tok;
        }
        '|' => {
            let tok: Token;
            if peek_char(lexer) == Option::Some('|') {
                tok = Token::OrOperation;
                lexer.position += 2;
                lexer.read_position += 2;
            } else {
                tok = Token::Pipe;
                lexer.position += 1;
                lexer.read_position += 1;
            }
            return tok;
        }
        '*' => {
            lexer.position += 1;
            lexer.read_position += 1;
            Token::Asterisk
        }
        '?' => {
            lexer.position += 1;
            lexer.read_position += 1;
            Token::QuestionMark
        }
        '+' => {
            lexer.position += 1;
            lexer.read_position += 1;
            Token::Plus
        }
        '\n' => {
            lexer.position += 1;
            lexer.read_position += 1;
            Token::EOL
        }
        '_' => {
            lexer.position += 1;
            lexer.read_position += 1;
            Token::Underscore
        }
        // TODO: Implement escaping double quotes in strings.
        '"' => {
            let Some(index) = lexer.input[lexer.position+1..]
                .iter()
                .position(|&c| c == '"')
                else {
                    error!("Literal string not terminated.");
                    panic!("No double quotes found");
                };
            let tok: Token = Token::Literal(lexer.input[lexer.position + 1..index + lexer.position + 1]
                .iter()
                .collect()
            );
            lexer.position = index + lexer.position + 2;
            lexer.read_position = lexer.position + 1;
            return tok;
        }
        // This matches for IDs and number literals.
        // Valid IDs have to start with alphabetic and have alphanumeric or numbers.
        // Number literals start with numbers
        ch => {
            let tok: Token;
            if ch.is_ascii_alphabetic() {
                let Some(index) = lexer.input[lexer.position..]
                    .iter()
                    .position(|&c| !(c.is_ascii_alphanumeric() || c == '_'))
                    else {
                        error!("Could not find non-alphanumeric ending.");
                        panic!("No newline token at end of file for variable.");
                    };
                tok = Token::ID(lexer.input[lexer.position..index + lexer.position]
                    .iter()
                    .collect()
                );
                lexer.position = index + lexer.position;
                lexer.read_position = lexer.position + 1;

                return tok;
            }
            if ch.is_numeric() {
                let Some(index) = lexer.input[lexer.position..]
                    .iter()
                    .position(|&c| !c.is_numeric())
                    else {
                        error!("Could not find non-numeric ending.");
                        panic!("No newline token at end of file for variable.");
                    };
                tok = Token::Literal(lexer.input[lexer.position..index + lexer.position]
                    .iter()
                    .collect()
                );
                lexer.position = index + lexer.position;
                lexer.read_position = lexer.position + 1;
                return tok;
            }
            // TODO: Implement centralised error handling.
            error!{"Invalid character \'{}\' detected.", ch};
            panic!{"Invalid Character set"};
        }
    }
}