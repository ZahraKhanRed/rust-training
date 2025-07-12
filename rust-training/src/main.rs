

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq)]
enum FizzBuzz {
    Fizz,
    Buzz,
    FizzBuzz,
    Number(u8)
}

fn fizz_buzz(n: u8) -> Vec<FizzBuzz> {
    let mut fizz_buzz_values: Vec<FizzBuzz> = vec![];

    for number in 1..=n {
        let fizz = number % 3 == 0;
        let buzz = number % 5 == 0;


        let value = match (fizz, buzz) {
            (true, true)            => FizzBuzz::FizzBuzz,
            (true, false) => FizzBuzz::Fizz,
            (false, true) => FizzBuzz::Buzz,
            (false, false) => FizzBuzz::Number(number),
        };


        fizz_buzz_values.push(value);
    };
    
    fizz_buzz_values
}

// count the numbers from 1 to 100
// when is multiuple of 3 is Fizz
// when is multiple of 5 is Buzz
// when is multiple of 3 and five is FizzBuzz
// lese print the number;

#[cfg(test)]
mod test {

    use crate::FizzBuzz;

    use super::fizz_buzz;

    #[test] 

    fn test_fizz_buzz() {
        let result = fizz_buzz(1);
        assert_eq!(result.len(), 1);
        assert_eq!(result, vec![FizzBuzz::Number(1)]);
    }

    #[test] 
    fn test_fizz_buzz_with_2() {
        let result = fizz_buzz(2);
        assert_eq!(result.len(), 2);
        assert_eq!(result, vec![FizzBuzz::Number(1), FizzBuzz::Number(2)]);
    }

    #[test] 
    fn test_fizz_buzz_with_3() {
        let result = fizz_buzz(3);
        assert_eq!(result.len(), 3);
        assert_eq!(result, vec![FizzBuzz::Number(1), FizzBuzz::Number(2), FizzBuzz::Fizz]);
    }

    #[test]
    fn test_fizz_buzz_with_5() {
        let result = fizz_buzz(5);
        assert_eq!(result.len(), 5);
        assert_eq!(result, vec![FizzBuzz::Number(1), FizzBuzz::Number(2), FizzBuzz::Fizz, FizzBuzz::Number(4), FizzBuzz::Buzz]);
    }

    #[test]
    fn test_fizz_buzz_with_6() {
        let result = fizz_buzz(6);
        assert_eq!(result.len(), 6);
        assert_eq!(result, vec![FizzBuzz::Number(1), FizzBuzz::Number(2), FizzBuzz::Fizz, FizzBuzz::Number(4), FizzBuzz::Buzz, FizzBuzz::Fizz]);
    }

    #[test]
    fn test_fizz_buzz_with_15() {
        let result = fizz_buzz(15);
        assert_eq!(result.len(), 15);
        assert_eq!(result, vec![FizzBuzz::Number(1), FizzBuzz::Number(2), FizzBuzz::Fizz, FizzBuzz::Number(4), FizzBuzz::Buzz, FizzBuzz::Fizz, FizzBuzz::Number(7), FizzBuzz::Number(8), FizzBuzz::Fizz, FizzBuzz::Buzz, FizzBuzz::Number(11), FizzBuzz::Fizz, FizzBuzz::Number(13), FizzBuzz::Number(14), FizzBuzz::FizzBuzz]);
    }
}