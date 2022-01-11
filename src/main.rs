#[allow(dead_code)]
use std::env;
use std::fs;
use colored::*;
use std::fmt;
use exitcode;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "help" {
        println!("Usage:Fuel <run or compile> <filepath>\n");
    }else{
    if args.len() < 3 {
        println!("{}","Please provide a filepath".red()); 
        std::process::exit(exitcode::USAGE);
    }else if args.len() > 3{
        println!("{}","Too many arguments".red());
        std::process::exit(exitcode::USAGE);
    }
    else{
        if args[1] == "run"{
            println!("{} code runing!\n","unoptimized".green());
            let lexer = Lexer{input: read_with_newline(&args[2]).to_vec()};
            let lexed = lexer.lex();
            _print_tokens(lexed);
            std::process::exit(exitcode::OK);
        }else if args[1] == "compile"{
            println!("building");
        }
    }
}
}
#[derive(Debug)]
pub enum TokenTypes{
    //dataTokenTypes
    ARRAY,
    STRING,
    TABLE,
    INTEGER,
    BOOL,
    DOUBLE,
    FLOAT,
    //functions
    PRINTFN,
    INPUT,
    //misc
    IDENTIFIER,
    CLASS,
    NEWLINE,
    RPAREN,
    LPAREN,
    //operators
    PLUS,
    MINUS,
    DIV,
    MUL,
    EQUALS,
    //statments
    IFEQUALS,
    LESSOREQUAL,
    MOREOREQUAL,
    BIGGERTHAN,
    SMALLERTHAN,
    NOT,
    OR,
    AND,
    //loops
    WHILE,
    FOR,
    IF,
    THEN,
    ACTION
}

//turning enums to string
impl fmt::Display for TokenTypes{
    fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            TokenTypes::LPAREN => write!(f,"LPAREN"),
            TokenTypes::RPAREN => write!(f,"RPAREN"),
            TokenTypes::NEWLINE => write!(f,"NEWLINE"), 
            TokenTypes::IDENTIFIER => write!(f,"IDENTIFIER"),
            TokenTypes::STRING => write!(f,"STRING"),
            TokenTypes::INTEGER => write!(f,"INTEGER"),
            TokenTypes::BOOL => write!(f,"BOOL"),
            TokenTypes::ARRAY => write!(f,"ARRAY"),
            TokenTypes::TABLE => write!(f,"TABLE"),
            TokenTypes::PRINTFN => write!(f,"PRINTFN"),
            TokenTypes::PLUS => write!(f,"PLUS"),
            TokenTypes::MINUS => write!(f,"MINUS"),
            TokenTypes::DIV => write!(f,"DIV"),
            TokenTypes::MUL => write!(f,"MUL"),
            TokenTypes::EQUALS => write!(f,"EQUALS"),
            TokenTypes::MOREOREQUAL => write!(f,"MOREOREQUAL"),
            TokenTypes::LESSOREQUAL => write!(f,"LESSOREQUAL"),
            TokenTypes::BIGGERTHAN => write!(f,"BIGGERTHAN"),
            TokenTypes::SMALLERTHAN => write!(f,"SMALLERTHAN"),
            TokenTypes::OR => write!(f,"OR"),
            TokenTypes::AND => write!(f,"AND"),
            TokenTypes::NOT => write!(f,"NOT"),
            TokenTypes::IFEQUALS => write!(f,"IFEQUALS"),
            TokenTypes::WHILE => write!(f,"WHILE"),
            TokenTypes::FOR => write!(f,"FOR"),
            TokenTypes::IF => write!(f,"IF"),
            TokenTypes::THEN => write!(f,"THEN"),
            TokenTypes::ACTION => write!(f,"ACTION"),
            TokenTypes::FLOAT => write!(f,"FLOAT"),
            TokenTypes::DOUBLE => write!(f,"DOUBLE"),
            TokenTypes::INPUT => write!(f,"INPUT"),
            TokenTypes::CLASS => write!(f,"CLASS"),
        }
    }
}

trait TOK{
    fn to_string(&self) -> String;
}

impl TOK for Token{
    fn to_string(&self) -> String{
        let mut a:String = String::new();
        a += &String::from(self.token_type.to_string());
        a += &String::from(", ");
        a += &self.token_value;
        return a;
    }
}

fn _print_tokens(inp:Vec<Token>){
    for i in inp{println!("[{}]",i.to_string())}
}

//important functions
fn read_with_newline(filepath:&String) -> Vec<String>{
    let mut res: Vec<String> = Vec::new();
    let contents = fs::read_to_string(filepath).expect("Something went wrong reading the file");
    let mut word:String = String::new();
    
    //importing newlines
    for i in 0..contents.chars().count(){
        match contents.chars().nth(i){
            Some(t) => {
                if String::from(t) == " "{
                    res.push(word.to_string());
                    word = String::new();
                }
                else if String::from(t) == "\n"{
                    res.push(word.to_string());
                    word = String::new();
                }
                else if String::from(t) == "\n\r"{
                    res.push(word.to_string());
                    word = String::new();
                }
                else{word += &String::from(t);}
            },
            _ => println!("No Char")
        }
    }
    
    return res;
}

#[derive(Debug)]
struct Token{
    token_type: TokenTypes,
    token_value: String,
}

//lexer
struct Lexer{
    input: Vec<String>,
}

trait LEXER{
    fn lex(&self) -> Vec<Token>;
}

impl LEXER for Lexer{
    fn lex(&self) -> Vec<Token>{
        let mut res: Vec<Token> = Vec::new();
        for i in &self.input{
            //datatypes
            if i == "String"{
                res.push(Token{ token_type: TokenTypes::STRING, token_value: String::from(i)});
            }else if i == "Array"{
                res.push(Token{ token_type: TokenTypes::ARRAY, token_value: String::from(i)});
            }else if i == "Integer"{
                res.push(Token{ token_type: TokenTypes::INTEGER, token_value: String::from(i)});
            }else if i == "Bool"{
                res.push(Token{ token_type: TokenTypes::BOOL, token_value: String::from(i)});
            }else if i == "Float"{
                res.push(Token{ token_type: TokenTypes::FLOAT, token_value: String::from(i)});
            }else if i == "Double"{
                res.push(Token{ token_type: TokenTypes::DOUBLE, token_value: String::from(i)});
            }else if i == &String::new(){
                res.push(Token{ token_type: TokenTypes::NEWLINE, token_value: String::from(i)})
            }
            //functions
            else if i == "input"{
                res.push(Token{ token_type: TokenTypes::INPUT, token_value: String::from(i)});
            }else if i == "print"{
                res.push(Token{ token_type: TokenTypes::PRINTFN, token_value: String::from(i)});
            }
            //misc
            else if i == "class"{
                res.push(Token{ token_type: TokenTypes::CLASS, token_value: String::from(i)});
            }
            else{
                res.push(Token{ token_type: TokenTypes::IDENTIFIER, token_value: String::from(i)});
            }
        }
        
        return res;
    }
}

//string slicing
use std::ops::{Bound, RangeBounds};

trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> &str;
    fn slice(&self, range: impl RangeBounds<usize>) -> &str;
}

impl StringUtils for String {
    fn substring(&self, start: usize, len: usize) -> &str {
        let mut char_pos = 0;
        let mut byte_start = 0;
        let mut it = self.chars();
        loop {
            if char_pos == start { break; }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_start += c.len_utf8();
            }
            else { break; }
        }
        char_pos = 0;
        let mut byte_end = byte_start;
        loop {
            if char_pos == len { break; }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_end += c.len_utf8();
            }
            else { break; }
        }
        &self[byte_start..byte_end]
    }
    fn slice(&self, range: impl RangeBounds<usize>) -> &str {
        let start = match range.start_bound() {
            Bound::Included(bound) | Bound::Excluded(bound) => *bound,
            Bound::Unbounded => 0,
        };
        let len = match range.end_bound() {
            Bound::Included(bound) => *bound + 1,
            Bound::Excluded(bound) => *bound,
            Bound::Unbounded => self.len(),
        } - start;
        self.substring(start, len)
    }
}

//parser
struct Parser {
    inp: Vec<Token>
}

trait Pars{
    fn parse(&self);
}

impl Pars for Parser{
    fn parse(&self){
        //variable detection
        let varlist: Vec<Variable> = Vec::new();
        for i in 0..self.inp.len(){
            let cur = &self.inp[i];
            if cur.token_type.to_string() == TokenTypes::STRING.to_string(){
                
            }
        }
    }
}

//utilities

struct Variable{
    vartype: TokenTypes,
    varname: String,
    varvar:String, 
}

trait Tostr{
    fn to_string(&self) -> String;
}

impl Tostr for Variable{
    fn to_string(&self) -> String{
        let mut res:String = String::new();
        res += &self.vartype.to_string();
        res += ", ";
        res += &self.varname;
        res += &self.varvar;
        return res;
    }
}