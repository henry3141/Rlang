use std::{env, string};
use std::fs::File;
use std::ops::{Index, IndexMut};
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

//create a list wich can hold diffrent types of data
//in the same object
#[derive(Debug,Clone,PartialEq,Eq)]
struct Node {
    data_type:String,
    data_string:Option<String>,
    data_i32:Option<i32>,
    data_usize:Option<usize>,
    data_bool:Option<bool>,
    data_node:Option<Box<Node>>,
    data_vec:Option<Vec<Node>>,
}

impl Node {
    fn new() -> Node {
        Node {
            data_type:String::from("None"),
            data_string:None,
            data_i32:None,
            data_usize:None,
            data_bool:None,
            data_node:None,
            data_vec:None,
        }
    }

    fn new_string(string:String) -> Node {
        Node {
            data_type:String::from("String"),
            data_string:Some(string),
            data_i32:None,
            data_usize:None,
            data_bool:None,
            data_node:None,
            data_vec:None,
        }
    }

    fn new_i32(i32:i32) -> Node {
        Node {
            data_type:String::from("i32"),
            data_string:None,
            data_i32:Some(i32),
            data_usize:None,
            data_bool:None,
            data_node:None,
            data_vec:None,
        }
    }

    fn new_usize(usize:usize) -> Node {
        Node {
            data_type:String::from("usize"),
            data_string:None,
            data_i32:None,
            data_usize:Some(usize),
            data_bool:None,
            data_node:None,
            data_vec:None,
        }
    }

    fn new_bool(bool:bool) -> Node {
        Node {
            data_type:String::from("bool"),
            data_string:None,
            data_i32:None,
            data_usize:None,
            data_bool:Some(bool),
            data_node:None,
            data_vec:None,
        }
    }

    fn new_node(node:Box<Node>) -> Node {
        Node {
            data_type:String::from("node"),
            data_string:None,
            data_i32:None,
            data_usize:None,
            data_bool:None,
            data_node:Some(node),
            data_vec:None,
        }
    }

    fn new_vec(vec:Vec<Node>) -> Node {
        Node {
            data_type:String::from("vec"),
            data_string:None,
            data_i32:None,
            data_usize:None,
            data_bool:None,
            data_node:None,
            data_vec:Some(vec),
        }
    }   
}

//---------------------------------
//This is a Lexer + Parser
//One Token represents a line of code.

struct Lexer {
    input:String,
    position:usize,
    current_char:Option<char>,
    output:Vec<Node>,
}

impl Lexer {
    fn new(input:String) -> Lexer {
        let mut lexer = Lexer {
            input:input,
            position:0,
            current_char:None,
            output:vec![],
        };
        lexer
    }

    fn advance(&mut self) {
        self.position += 1;
        if self.position > self.input.len() - 1 {
            self.current_char = None;
        } else {
            self.current_char = Some(self.input.chars().nth(self.position).unwrap());
        }
    }

    fn next(&self) -> char {
        if self.position > self.input.len() - 1 {
            '\0'
        } else {
            self.input.chars().nth(self.position).unwrap()
        }
    }

    fn skip_comment(&mut self) {
        while self.current_char.is_some() && (self.current_char.unwrap() != '\n' ) {
            self.advance();
        }
        self.advance();
    }

    fn check_next(&self,string:&str) -> bool {
        if self.position > self.input.len() - 1 {
            return false;
        }
        let mut index = 0;
        while index < string.len() {
            if self.input.chars().nth(self.position + index).unwrap() != string.chars().nth(index).unwrap() {
                return false;
            }
            index += 1;
        }
        return true;
    }

    fn multi_advance(&mut self,count:usize) {
        for _ in 0..count {
            self.advance();
        }
    }

    fn get_word(&mut self) -> String {
        let mut word = String::from("");
        while self.current_char.is_some() && self.current_char.unwrap().is_alphanumeric() {
            word.push(self.current_char.unwrap());
            self.advance();
        }
        word
    }

    fn lexer(&mut self) {
        //Tokens :
        // 7: == = equal
        // 8: != = not equal
        // 9: < = less than
        // 10: > = greater than
        // 11: <= = less than or equal to
        // 12: >= = greater than or equal to
        // 13: ( = open parenthesis
        // 14: ) = close parenthesis
        // 15: { = open curly brace
        // 16: } = close curly brace
        // 17: , = comma
        // 18: ; = semicolon
        self.input = smart_replace(&self.input, &'\r', &'\0');
        self.position = 0;
        self.current_char = Some(self.input.chars().nth(self.position).unwrap());
        while self.current_char.is_some() {
            let mut node = Node::new();
            if self.current_char.unwrap() == ' ' || self.current_char.unwrap() == '\t' {
                self.advance();
                continue;
            } else if self.check_next("//") {
                self.skip_comment();
                continue;
            } else if self.check_next("==") {
                node.data_string = Some(String::from("=="));
                node.data_type = String::from("math");
                self.advance();

            } else if self.check_next("!=") {
                node.data_string = Some(String::from("!="));
                node.data_type = String::from("math");
                self.advance();

            } else if self.check_next("<") {
                node.data_string = Some(String::from("<"));
                node.data_type = String::from("math");
                self.advance();

            } else if self.check_next(">") {
                node.data_string = Some(String::from(">"));
                node.data_type = String::from("math");
                self.advance();

            } else if self.check_next("<=") {
                node.data_string = Some(String::from("<="));
                node.data_type = String::from("math");
                self.advance();

            } else if self.check_next(">=") {
                node.data_string = Some(String::from(">="));
                node.data_type = String::from("math");
                self.advance();

            } else if self.current_char.unwrap() == '"' {
                println!("String");
                let mut temp = String::from("");
                self.advance();
                while self.current_char.is_some() && (self.current_char.unwrap() != '"') {
                    temp.push(self.current_char.unwrap());
                    self.advance();
                }
                node.data_string = Some(temp);
                node.data_type = String::from("string");
            } else if self.current_char.unwrap() == '+' {
                node.data_string = Some(String::from("+"));
                node.data_type = String::from("math");

            } else if self.current_char.unwrap() == '-' {
                node.data_string = Some(String::from("-"));
                node.data_type = String::from("math");

            } else if self.current_char.unwrap() == '*' {
                node.data_string = Some(String::from("*"));
                node.data_type = String::from("math");

            } else if self.current_char.unwrap() == '/' {
                node.data_string = Some(String::from("/"));
                node.data_type = String::from("math");

            } else if self.current_char.unwrap() == '%' {
                node.data_string = Some(String::from("&"));
                node.data_type = String::from("math");

            } else if self.current_char.unwrap().is_digit(10) && self.next().is_digit(10) {
                let mut number = String::from("");
                while self.current_char.is_some() && self.current_char.unwrap().is_digit(10) {
                    number.push(self.current_char.unwrap());
                    self.advance();
                }
                node.data_i32 = Some(number.parse::<i32>().unwrap());
                node.data_type = String::from("number");

            } else if self.check_next("let") {
                node.data_string = Some(String::from("let"));
                node.data_type = String::from("keyword");
                self.multi_advance(1);
            } else if self.check_next("if") {
                node.data_string = Some(String::from("if"));
                node.data_type = String::from("keyword");
                self.advance();
            } else if self.check_next("else") {
                node.data_string = Some(String::from("else"));
                node.data_type = String::from("keyword");
                self.multi_advance(2);
            } else if self.check_next("while") {
                node.data_string = Some(String::from("while"));
                node.data_type = String::from("keyword");
                self.multi_advance(2);
            } else if self.check_next("print") {
                node.data_string = Some(String::from("print"));
                node.data_type = String::from("keyword");
                self.multi_advance(3);
            } else if self.check_next("return") {
                node.data_string = Some(String::from("return"));
                node.data_type = String::from("keyword");
                self.multi_advance(4);
            } else if self.check_next("true") {
                node.data_bool = Some(true);
                node.data_type = String::from("boolean");
            } else if self.check_next("false") {
                node.data_bool = Some(false);
                node.data_type = String::from("boolean");
            } else {
                //fix for empty string
                node.data_string = Some(self.get_word());
                node.data_type = String::from("unidentiefied_word");
            }

            if (node.data_type != String::from("")) && !(node.data_string == Some(String::from("")) && node.data_type == String::from("unidentiefied_word")) {
                self.output.push(node);
            }

            self.advance();
        }
    }
}









//---------------------------------

fn main() {
    let args:Vec<String> = env::args().collect();
    if args.len() == 1 {
        error("FileExecuteError: No file specified");
    }
    let mut file = File::open(&args[1]).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut lexer = Lexer::new(contents);
    lexer.lexer();
    println!("{:#?}",lexer.output);
}