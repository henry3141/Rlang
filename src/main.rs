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

    fn smart_split(string:&str,split:&str) -> Vec<String> {
        //This function splits a string into a vector of strings using a split string.
        //But it doesnt split inbetween "" and ''

        let mut output = Vec::new();
        let mut temp = String::new();
        let mut in_string = false; // a string is inbetween ""
        let mut in_string_2 = false; // a string 2 starts with ' and ends with '
        let mut in_multi_line_comment = false; // a multiline comment starts with /* and ends with */


        for c in string.chars() {
            if in_multi_line_comment {
                if c == '*' && string.chars().nth(1) == Some('/') {
                    in_multi_line_comment = false;
                }
            } else if in_string {
                if c == '"' {
                    in_string = false;
                }
            } else if in_string_2 {
                if c == '\'' {
                    in_string_2 = false;
                }
            } else if c == '"' {
                in_string = true;
            } else if c == '\'' {
                in_string_2 = true;
            } else if c == '/' && string.chars().nth(1) == Some('*') {
                in_multi_line_comment = true;
            } else if c == split.chars().nth(0).unwrap() {
                let mut con = true;
                for i in 0..split.len() {
                    if string.chars().nth(i) != Some(split.chars().nth(i).unwrap()) {
                        con = false;
                        break;
                    }
                }
                if con == true {
                    output.push(temp);
                    temp = String::new();
                    continue;
                }
            }
            temp.push(c);
        }

        if temp.len() > 0 {
            output.push(temp);
        }
        output
    }

    fn smart_replace(string:&str,replace:&str,with:&str) -> String {
        //This function replaces a string with another string using a replace string.
        //But it doesnt replace inbetween "" and ''

        let mut output = String::new();
        let mut in_string = false; // a string is inbetween ""
        let mut in_string_2 = false; // a string 2 starts with ' and ends with '
        let mut in_multi_line_comment = false; // a multiline comment starts with /* and ends with */
        let mut jump = 0;

        for i in 0..string.len() {
            if jump != 0 {
                jump -= 1;
                continue;
            } else if in_multi_line_comment == true {
                if string.chars().nth(i) == Some('*') && string.chars().nth(i+1) == Some('/') {
                    in_multi_line_comment = false;
                    jump = 1;
                }
            } else if in_string == true {
                if string.chars().nth(i) == Some('"') {
                    in_string = false;
                }
            } else if in_string_2 == true {
                if string.chars().nth(i) == Some('\'') {
                    in_string_2 = false;
                }
            } else if string.chars().nth(i) == Some('"') {
                in_string = true;
            } else if string.chars().nth(i) == Some('\'') {
                in_string_2 = true;
            } else if string.chars().nth(i) == Some('/') && string.chars().nth(i+1) == Some('*') {
                in_multi_line_comment = true;
            } else if string.chars().nth(i) == Some(replace.chars().nth(0).unwrap()) {
                let mut con = true;
                for j in 0..replace.len() {
                    if string.chars().nth(i+j) != Some(replace.chars().nth(j).unwrap()) {
                        con = false;
                        break;
                    }
                }
                if con == true {
                    output.push_str(with);
                    jump = replace.len()-1;
                } else {
                    output.push(string.chars().nth(i).unwrap());
                }
            }
            output.push(string.chars().nth(i).unwrap());
        }
        output
    }

    fn lexer(&mut self) {
        self.input = Lexer::smart_replace(&self.input,"\r","");
        self.input = Lexer::smart_replace(&self.input,"\t","");
        let mut input = Lexer::smart_split(&self.input,"\n");
        
    }

}

fn main() {
}