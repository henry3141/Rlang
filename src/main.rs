//Main file of Rlang
//A progrmming language coded in Rust

//First a Lexer
//A lexer is a program that takes a stream of characters and breaks it into a organized stream of tokens.

//One Token represents a line of code.
struct Token {
    //Kind holds a name used in the intepreter to identify the token
    kind: String,
    //Value holds things like the args in a function definition and the name of a variable
    data:Vec<String>,
    //Line holds the line number of the token
    line:usize,
    //Under is a vector of tokens that are children of the token like in a function definition the Code to be executed when the function is called 
    under:Vec<Token>
}



struct Lexer {
    input: String,
    output: Vec<Token>,
    line: usize,
}

impl Lexer {
    fn new(input: String) -> Lexer {
        Lexer {
            input: input,
            output: Vec::new(),
            line: 0,
        }
    }

    fn Lexer() {
        //TODO
    }

}

fn main() {}