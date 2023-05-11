pub mod interpreter;

#[cfg(test)]
mod tests {
    use super::interpreter::{
        lexer::tokenize,
        parser::{evaluate, parse},
    };

    #[test]
    fn add() {
        let input = "1 + 2";
        let tokens = tokenize(&input);
        let expr = parse(&tokens);
        let res = evaluate(&expr);

        assert_eq!(res, 3.0);
    }

    #[test]
    fn sub() {
        let input = "3 - 2";
        let tokens = tokenize(&input);
        let expr = parse(&tokens);
        let res = evaluate(&expr);

        assert_eq!(res, 1.0);
    }

    #[test]
    fn mul() {
        let input = "2 * 2";
        let tokens = tokenize(&input);
        let expr = parse(&tokens);
        let res = evaluate(&expr);

        assert_eq!(res, 4.0);
    }

    #[test]
    fn div() {
        let input = "8 / 2";
        let tokens = tokenize(&input);
        let expr = parse(&tokens);
        let res = evaluate(&expr);

        assert_eq!(res, 4.0);
    }

    #[test]
    fn order_of_ops() {
        let input = "2 * 4 ( 1 + 2 )";
        let tokens = tokenize(&input);
        let expr = parse(&tokens);
        let res = evaluate(&expr);

        assert_eq!(res, 24.0);
    }

    #[test]
    fn nested_parens() {
        let input = "12 ( 2 ( 2 ) )";
        let tokens = tokenize(&input);
        let expr = parse(&tokens);
        let res = evaluate(&expr);

        assert_eq!(res, 48.0);
    }

    #[test]
    fn very_nested_parens() {
        let input = "2 ( ( ( ( 2 ) ) ) )";
        let tokens = tokenize(&input);
        let expr = parse(&tokens);
        let res = evaluate(&expr);

        assert_eq!(res, 4.0);
    }
}
