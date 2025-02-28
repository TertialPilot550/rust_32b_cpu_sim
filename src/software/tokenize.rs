const OPERATORS: &str = "+-*/=";
const PUNCTUATION: &str = ",;:()[]{}";

// Given an input string, returns a vector of tokens in that string
pub fn tokenize(input: String) -> Vec<Token> {
    let mut tkn = Tokenizer::new(input.clone());
    let mut res = Vec::<Token>::new();
    let mut i = 0;
    // extract all the tokens
    while tkn.has_next() {
        let token = tkn.next().unwrap();

        println!("TOKEN #{i}:[{}]", token.value);
        res.push(token);
        i += 1;
    };

    res
}

// Possible types of tokens for easier parsing later
pub enum TokenType {
    Integer, Identifier, Operator, Punctuation, Other
}

// Output of the tokenizer
pub struct Token {
    token_type: TokenType,
    value: String
}

// Object to parse a String for tokens
pub struct Tokenizer {
    input_string: String,
    index: usize
}
impl Tokenizer {

    // Constructor
    fn new(input: String) -> Tokenizer{
        Tokenizer {
            input_string: input,
            index: 0
        }
    }

    // Return an Option of Token that contains the next bit of text in the input_string
    pub fn next(&mut self) -> Option<Token> {
        println!("Finding next @ {}", self.index);
        if !self.has_next() {
            return None
        }

        self.input_string = String::from(self.input_string.split_off(self.index).trim());
        self.index = 0;
        println!("remaining: [{}]", self.input_string);

        let current_char = self.input_string.chars().nth(self.index).unwrap();

        // it's an identifier
        if current_char.is_alphabetic() {
            let identifier = self.read_identifier();
            return Some ( Token { token_type: TokenType::Identifier, value: identifier })
        // it's a number
        } else if current_char.is_ascii_digit() {
            let number = self.read_number();
            self.index += number.len();
            return Some ( Token { token_type: TokenType::Integer, value: number})
       
        // it's white space
        } else if current_char.is_whitespace() {
            let whitespace = self.read_whitespace();
            self.index += whitespace.len();
            return self.next()

        // it's an operator
        } else if OPERATORS.contains(current_char) {
            self.index += 1;
            return Some ( Token { token_type: TokenType::Operator, value: current_char.to_string()})

        // it's punctuation
        } else if PUNCTUATION.contains(current_char) {
            self.index += 1;

            return Some ( Token { token_type: TokenType::Punctuation, value: current_char.to_string()})

        // it's some other shit
        } else {
            self.index += 1;

            return Some ( Token { token_type: TokenType::Other, value: current_char.to_string()})
        }

    }

    // returns true if there is more text to parse
    fn has_next(&self) -> bool {
        return self.index < self.input_string.len()
    }
   
    // call when you see letter: advance to the next non-letter character and get the word
    fn read_identifier(&mut self) -> String {
        let start = self.index;
        while self.index < self.input_string.len() {
            let current_char = self.input_string.chars().nth(self.index).unwrap();
            println!("Reading identifier [{current_char}]");


            if !current_char.is_alphabetic() {
                println!("BREAK!");
                break
            }

            
            self.index += 1;
        }
        println!("Start:{}, End:{}", start, self.index);
        return String::from(&self.input_string[start..self.index]);
    }

    // call when you see number: advance to the next non-number character and get the number
    fn read_number(&self) -> String {
        let mut i = 0;
        while self.index + i < self.input_string.len() {
            let current_char = self.input_string.chars().nth(self.index + i).unwrap();
            println!("Reading number [{current_char}]");

            if !(current_char.is_ascii_hexdigit() || (current_char == 'x' && i == 1)) {
                break
            }
            
            i += 1;
        }
        return String::from(&self.input_string[self.index..(self.index + i)]);
    }

    // call when you see whitespace: advance to the next non-whitespace character and get the whitespace
    fn read_whitespace(&self) -> String {
        let mut i = 0;
        while self.index + i < self.input_string.len() {
            let current_char = self.input_string.chars().nth(self.index + i).unwrap();
            println!("Reading whitespace [{current_char}]");
            if !current_char.is_alphabetic() {
                break
            }
            i += 1;
        }
        return String::from(&self.input_string[self.index..(self.index + i)]);
    }

}



