mod checked {
    #[derive(Debug)]
    enum MathError {
        DivisionByZero,
        NonPositiveLogarithm,
        NegativeSquareRoot,
    }

    type MathResult = Result<f64, MathError>;

    fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(x / y)
        }
    }

    fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    fn ln(x: f64) -> MathResult {
        if x <= 0.0 {
            Err(MathError::NonPositiveLogarithm)
        } else {
            Ok(x.ln())
        }
    }

        // 中間機能
        fn op_(x: f64, y: f64) -> MathResult {
            // もし`div`が 失敗したら、`DivisionByZero`がリターンされます。
            let ratio = div(x, y)?;
    
            // `ln` が失敗した場合は、`NonPositiveLogarithm` がリターンされます。
            let ln = ln(ratio)?;
    
            sqrt(ln)
        }
    
        pub fn op(x: f64, y: f64) {
            match op_(x, y) {
                Err(why) => panic!(match why {
                    MathError::NonPositiveLogarithm
                        => "logarithm of non-positive number",
                    MathError::DivisionByZero
                        => "division by zero",
                    MathError::NegativeSquareRoot
                        => "square root of negative number",
                }),
                Ok(value) => println!("{}", value),
            }
        }
    }

fn main() {
    checked::op(1.0, 16.0);
}
