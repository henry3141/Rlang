use std::env;
use std::fs::File;
use std::process;
use std::io::Read;

//Main file of Rlang
//A progrmming language coded in Rust


//---------------------------------
//This is a Lexer + Parser
//A lexer is a program that takes a stream of characters and breaks it into a organized stream of tokens.
//One Token represents a line of code.


#[derive(Debug, Clone)]
struct Token {
    kind: String,
    data:Vec<String>,
    line:usize,
    under:Vec<Token>
}

impl Token {
    fn empty() -> Token {
        Token {
            kind: "".to_string(),
            data: vec![],
            line: 0,
            under: vec![]
        }
    }

    fn add_under(&mut self,level:&i32,token:&Token) {
        if *level == 0 {
            self.under.push(token.to_owned());
        } else {
            if self.under.len() > 0 {
                let index:usize = self.under.len()-1;
                self.under[index].add_under(&(level-1),token);
            }
        }
    }
}



struct Lexer {
    input: String,
    output: Vec<Token>,
}

impl Lexer {
    fn new(input: String) -> Lexer {
        Lexer {
            input: input,
            output: Vec::new(),
        }
    }

    fn smart_replace(str:&str,char1:&char,char2:&char) -> String {
        let mut result:String = str.to_string();
        let mut index:usize = 0;
        while index < result.len() {
            if result.chars().nth(index).unwrap() == *char1 {
                result.remove(index);
                result.insert(index,*char2);
            }
            index += 1;
        }
        return result;
    }
    
    fn smart_split(str:&str,char1:&char) -> Vec<String> {
        let mut mainvec = vec![];
        let mut temp = "".to_string();	
        let mut block = false;
        for i in str.chars() {
            if i == *char1 && block == false {
                mainvec.push(temp.clone());
                temp = "".to_string();
            } else if i == '"' {
                block = !block;
                temp.push('"');
            } else {
                temp.push(i);
            }
        }
        if temp != "" {
            mainvec.push(temp);
        }
        mainvec
    }

    fn lexer(&mut self) {
        self.input = Lexer::smart_replace(&self.input,&'\r',&'\0');
        self.input = Lexer::smart_replace(&self.input,&'\t',&'\0');
        let mut input = Lexer::smart_split(&self.input,&'\n');
        let mut line:usize = 0;
        let mut level = 0;
        for i in &input {
            line += 1;
            let mut current_token = Token::empty();
            current_token.line = line; 
            if i.starts_with("/") || i == "" {
                continue;
            } else if i.starts_with("write") {
                current_token.kind = "keyword_write".to_string();
                let ivec = Lexer::smart_split(i,&' ');
                let args = Lexer::smart_split(&ivec[1].to_string(),&',');
                for j in &args {
                    current_token.data.push(j.to_string());
                }
            }

            if level == 0 {
                self.output.push(current_token);
            } else {
                let index = self.output.len()-1;
                self.output[index].add_under(&level,&current_token);
            }
        }
    }

}

//---------------------------------
//Then a Interpreter

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("FileError: No file specified");
        process::exit(0);
    }
    let mut file = File::open(&args[1]).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut lexer = Lexer::new(contents);
    lexer.lexer();
    println!("{:#?}",lexer.output);
}