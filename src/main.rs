
fn main() {
    _ = calculate("1 + 3 * (3-1) / 2").unwrap();
}

#[derive(Debug, PartialEq)]
enum Token
{
    Number(i32),
    Add,
    Subtract,
    Multiply,
    Divide,
    BeginParenthesis,
    EndParenthesis,
}
#[derive(Debug, PartialEq)]
enum CalculatorError{
    InvalidCharacter(char)
}
fn calculate(input: &str) -> Result<i32, CalculatorError>
{
    _ = tokenize(input)?;
    return Result::Ok(3);
}

fn tokenize(input: &str) -> Result<Vec<Token>, CalculatorError> {
    let mut tokens = vec![];
    let mut current_number: Option<i32> = Option::None;
    for char in input.chars() {
       match char { 
           '1' => { current_number = Option::Some(current_number.unwrap_or_else(|| 0) * 10 + 1)}
           '2' => { current_number = Option::Some(current_number.unwrap_or_else(|| 0) * 10 + 2)}
           '3' => { current_number = Option::Some(current_number.unwrap_or_else(|| 0) * 10 + 3)}
           '4' => { current_number = Option::Some(current_number.unwrap_or_else(|| 0) * 10 + 4)}
           '5' => { current_number = Option::Some(current_number.unwrap_or_else(|| 0) * 10 + 5)}
           '6' => { current_number = Option::Some(current_number.unwrap_or_else(|| 0) * 10 + 6)}
           '7' => { current_number = Option::Some(current_number.unwrap_or_else(|| 0) * 10 + 7)}
           '8' => { current_number = Option::Some(current_number.unwrap_or_else(|| 0) * 10 + 8)}
           '9' => { current_number = Option::Some(current_number.unwrap_or_else(|| 0) * 10 + 9)}
           '0' => { current_number = Option::Some(current_number.unwrap_or_else(|| 0) * 10 + 0)}
           '+' => { if current_number.is_some() { tokens.push(Token::Number(current_number.unwrap())); current_number = Option::None; }  tokens.push(Token::Add); }
           '-' => { if current_number.is_some() { tokens.push(Token::Number(current_number.unwrap())); current_number = Option::None; }  tokens.push(Token::Subtract); }
           '*' => { if current_number.is_some() { tokens.push(Token::Number(current_number.unwrap())); current_number = Option::None; }  tokens.push(Token::Multiply); }
           '/' => { if current_number.is_some() { tokens.push(Token::Number(current_number.unwrap())); current_number = Option::None; }  tokens.push(Token::Divide); }
           '(' => { if current_number.is_some() { tokens.push(Token::Number(current_number.unwrap())); current_number = Option::None; }  tokens.push(Token::BeginParenthesis); }
           ')' => { if current_number.is_some() { tokens.push(Token::Number(current_number.unwrap())); current_number = Option::None; }  tokens.push(Token::EndParenthesis); }
           ' ' => {} // Skip any white spaces
           _ => return Result::Err(CalculatorError::InvalidCharacter(char))
       } 
    }
    if current_number.is_some() { 
        tokens.push(Token::Number(current_number.unwrap())); 
    }
   return Result::Ok(tokens); 
} 

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    
    #[test]
    fn tokenize_numbers_in_beginning_and_end_test()
    {
       let tokens = tokenize("1 + 232*32-5/2 + 21").unwrap();

        assert_eq!(11, tokens.len());
        assert_eq!(Token::Number(1), tokens[0]);
        assert_eq!(Token::Add, tokens[1]);
        assert_eq!(Token::Number(232), tokens[2]);
        assert_eq!(Token::Multiply, tokens[3]);
        assert_eq!(Token::Number(32), tokens[4]);
        assert_eq!(Token::Subtract, tokens[5]);
        assert_eq!(Token::Number(5), tokens[6]);
        assert_eq!(Token::Divide, tokens[7]);
        assert_eq!(Token::Number(2), tokens[8]);
        assert_eq!(Token::Add, tokens[9]);
        assert_eq!(Token::Number(21), tokens[10]);
    }
   
    #[test]
    fn tokenize_non_numbers_in_beginning_and_end_test()
    {
        let tokens = tokenize("(1 + 232*(32)-").unwrap();

        assert_eq!(9, tokens.len());
        assert_eq!(Token::BeginParenthesis, tokens[0]);
        assert_eq!(Token::Number(1), tokens[1]);
        assert_eq!(Token::Add, tokens[2]);
        assert_eq!(Token::Number(232), tokens[3]);
        assert_eq!(Token::Multiply, tokens[4]);
        assert_eq!(Token::BeginParenthesis, tokens[5]);
        assert_eq!(Token::Number(32), tokens[6]);
        assert_eq!(Token::EndParenthesis, tokens[7]);
        assert_eq!(Token::Subtract, tokens[8]);
    }
    #[test]
    fn tokenize_all_digits_test()
    {
        let tokens = tokenize("1234567890").unwrap();

        assert_eq!(1, tokens.len());
        assert_eq!(Token::Number(1234567890), tokens[0]);
    }
    
    #[test]
    fn tokenize_invalid_char_expect_error_test()
    {
        let result = tokenize("1 +M 232*32-  21");
        assert_eq!( Result::Err(CalculatorError::InvalidCharacter('M')), result);
    }
}

