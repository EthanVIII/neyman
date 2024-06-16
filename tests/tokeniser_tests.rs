
//! These are tests for the tokeniser component of the parser.
mod tests {
    use std::panic::catch_unwind;
    use neyman::parse::*;
    
    #[test]
    fn parse_keywords() {
        let x: Vec<char> = "\
        if return test_var else_var else
        \
        ".chars().collect();
        let output: Vec<Token> = tokenise(x);
        assert_eq!(output,
                   vec![Token::IfToken,
                        Token::ReturnToken,
                        Token::ID(String::from("test_var")),
                        Token::ID(String::from("else_var")),
                        Token::ElseToken,
                        Token::EOL]
        )
    }
    
    #[test]
    fn invalid_chars() {
        let x: Vec<char> = "~".chars().collect();
        let result = catch_unwind( || {tokenise(x)});
        assert!(result.is_err());
    }
    
    #[test]
    fn parse_literals() {
        let x: Vec<char> = "\"Hello World+\" 1027384 \n".chars().collect();
        let result = tokenise(x);
        assert_eq!(result,
        vec![
            Token::Literal(String::from("Hello World+")),
            Token::Literal(String::from("1027384")),
            Token::EOL
        ]);
    }
    
    #[test]
    fn parse_ids() {
        let x: Vec<char> = "id_1 m1 1m _m\n".chars().collect();
        let result = tokenise(x);
        assert_eq!(result,
                   vec![
                       Token::ID(String::from("id_1")),
                       Token::ID(String::from("m1")),
                       Token::Literal(String::from("1")),
                       Token::ID(String::from("m")),
                       Token::Underscore,
                       Token::ID(String::from("m")),
                       Token::EOL
                   ]);
    }
    
    
}