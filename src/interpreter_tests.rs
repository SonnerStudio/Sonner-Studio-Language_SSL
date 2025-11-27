#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;

    fn evaluate(source: &str) -> Value {
        let mut parser = Parser::new(source);
        let ast = parser.parse().unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.interpret(ast).unwrap()
    }

    #[test]
    fn test_arithmetic() {
        assert_eq!(evaluate("1 + 2").as_int().unwrap(), 3);
        assert_eq!(evaluate("10 - 4").as_int().unwrap(), 6);
        assert_eq!(evaluate("3 * 4").as_int().unwrap(), 12);
        assert_eq!(evaluate("20 / 5").as_int().unwrap(), 4);
    }

    #[test]
    fn test_variables() {
        let source = "
            let x = 10
            let y = 20
            x + y
        ";
        assert_eq!(evaluate(source).as_int().unwrap(), 30);
    }

    #[test]
    fn test_function_call() {
        let source = "
            fn add(a: Int, b: Int) -> Int {
                return a + b
            }
            add(5, 7)
        ";
        assert_eq!(evaluate(source).as_int().unwrap(), 12);
    }

    #[test]
    fn test_recursion() {
        let source = "
            fn fib(n: Int) -> Int {
                if n <= 1 { return n }
                return fib(n-1) + fib(n-2)
            }
            fib(6)
        ";
        assert_eq!(evaluate(source).as_int().unwrap(), 8);
    }
}
