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

    fn smart_replace(str:&str,char1:&char,char2:&char) -> String {
        let mut re = "".to_string();
        let mut block = false;
        for i in str.chars() {
            if i == *char1 && block == false {
                if char2 != &'\0' {
                    re.push(*char2);
                }
            } else if i == '"' {
                block = !block;
                re.push('"');
            } else {
                re.push(i);
            }
        }
        re
    }
    
    fn smart_split(str:&str,char1:&char) -> Vec<String> {
        let mut re:Vec<String> = vec!["".to_string()];
        let mut block = false;
        for i in str.chars() {
            if i == *char1 && block == false {
                re.push("".to_string());
            } else if i == '"' {
                block = !block;
                let index:usize = (re.len() - 1).try_into().unwrap();
                re[index].push('"');
            } else {
                let index:usize = (re.len() - 1).try_into().unwrap();
                re[index].push(i);
            }
        }
        re
    }

    fn lexer(&mut self) {
        self.input = Lexer::smart_replace(&self.input,&'\r',&'\0');
        self.input = Lexer::smart_replace(&self.input,&'\t',&'\0');
        let mut input = Lexer::smart_split(&self.input,&'\n');
        println!("{:?}",input);
    }

}

fn main() {
    let mut input = String::from("print('Doing varios tests\nTest1:LOL')\nSecond Line");
    let mut lexer = Lexer::new(input);
    lexer.lexer();
}