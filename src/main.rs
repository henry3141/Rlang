use std::env;
use std::fs::File;
use std::process;
use std::io::Read;
use std::collections::HashMap;

//Main file of Rlang
//A progrmming language coded in Rust

//---------------------------------
//lib

fn error(message:&str) {
    println!("{}",message);
    process::exit(0);
}


//---------------------------------
//This is a Lexer + Parser
//A lexer is a program that takes a stream of characters and breaks it into a organized stream of tokens.
//One Token represents a line of code.


#[derive(Debug, Clone)]
struct Token {
    kind: String,
    data:Vec<String>,
    line:usize,
    under:Vec<Token>,
}

impl Token {
    fn empty() -> Token {
        Token {
            kind: "".to_string(),
            data: vec![],
            line: 0,
            under: vec![],
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
        let mut result:Vec<char> = str.chars().collect();
        let mut index:usize = 0;
        let mut block = false;
        while index < result.len() {
            if result[index] == *char1 && block == false {
                result.remove(index);
                if char2 != &'\0' {
                    result.insert(index,*char2);
                }
            } else if result[index] == '"' {
                block = !block;
            }
            index += 1;
        }
        return result.iter().collect();
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
            }  else if i .starts_with("let") {
                current_token.kind = "keyword_let".to_string();
                let ivec = Lexer::smart_split(i,&' ');
                let args = ivec[1..].to_vec().join("");
                let args = Lexer::smart_replace(&args, &' ', &'\0');
                let args = Lexer::smart_split(&args,&'=');
                current_token.data.push(args[0].to_string());
                current_token.data.push(args[1].to_string());
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
//An interpreter is a program that takes a stream of tokens and executes it.

#[derive(Debug, Clone)]
struct Variable {
    kind: String,
    data: String,
    token_data: Token,
    name: String,
}

impl Variable {
    fn new(kind: String, data: String, token_data: Token, name: String) -> Variable {
        Variable {
            kind: kind,
            data: data,
            token_data: token_data,
            name: name,
        }
    }

    fn get_data(&self) -> String {
        self.data.clone()
    }

    fn empty() -> Variable {
        Variable {
            kind: "".to_string(),
            data: "".to_string(),
            token_data: Token::empty(),
            name: "".to_string(),
        }
    }

    fn function_preset(name: String, token_data: Token) -> Variable {
        Variable::new("function".to_string(), "".to_string(), token_data, name)
    }

    fn variable_preset(name: String, data: String , kind:String) -> Variable {
        Variable::new(kind, data, Token::empty(), name)
    }

}

struct Interpreter {
    input: Vec<Token>,
    variables: HashMap<String, Vec<Variable>>,
    line: usize,
    namespace: Vec<String>,
}

impl Interpreter {
    fn new(input: Vec<Token>) -> Interpreter {
        let mut i = Interpreter {
            input: input,
            variables: HashMap::new(),
            line: 0,
            namespace: vec!["Global".to_string()],
        };
        i.variables.insert("Global".to_string(), vec![]);
        i
    }

    fn get_variable(&self,string:String) -> Variable {
        //will turn any string it gets into a variable of the right type
        //or find it in self.variables
        if string.starts_with("\"") && string.ends_with("\"") {
            return Variable::variable_preset("internal_var".to_owned(), string[1..string.len()-1].to_string() , "string".to_owned());
        } else {
            for i in &self.variables {
                if self.namespace.contains(&i.0) {
                    for j in i.1 {
                        if j.name == string {
                            return j.clone();
                        }
                    }
                }
            }
            error(&("[".to_owned().to_owned() + &self.line.to_string() +"]: UnknownTypeError"));
        }
        Variable::empty()
    } 

    fn execute(&mut self) {
        let mut TokenStream:Vec<Token> = self.input.clone();
        let mut index:usize = 0;
        while index < TokenStream.len() {
            let mut i = TokenStream[index].clone();
            self.line = i.line;
            if i.kind == "keyword_write" {
                let mut data = "".to_string();
                for j in &i.data {
                    let temp = &self.get_variable(j.clone());
                    if temp.kind == "string".to_string() {
                        data.push_str(&temp.get_data());
                    } else {
                        error(&("[".to_owned()+ &i.line.to_string() + &"]: TypeError: expected string, got ".to_string() + &temp.kind));
                    }
                }
                print!("{}",data.replace("\\n","\n"));
            } else if i.kind == "keyword_let" {
                //create a variable and append it to the current namespace
                let mut name = i.data[0].clone();
                let mut data = i.data[1].clone();
                let mut var = self.get_variable(data.clone());
                var.name = name.clone();
                self.variables.get_mut(&self.namespace[self.namespace.len()-1]).unwrap().push(var);
            }
            index += 1;
        }
    }
}


//---------------------------------

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        error("FileError: No file specified")
    }
    let mut file = File::open(&args[1]).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut lexer = Lexer::new(contents);
    lexer.lexer();
    println!("{:#?}",&lexer.output);
    let mut interpreter = Interpreter::new(lexer.output);
    interpreter.execute();
}