#[allow(dead_code)]
//libs
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
            TokenTypes::COMMENT => write!(f,"COMMENT"),
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
    
    for i in contents.lines(){
        for b in i.split_whitespace(){
            res.push(b.to_string());
        }
        res.push("NewLine".to_string());
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

#[derive(Debug)]
#[derive(PartialEq)]
pub enum TokenTypes{
    //datatypes
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
    COMMENT,
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
            }else if i.chars().count() > 5{
                if i[0..5] == String::from("print"){
                    res.push(Token{ token_type: TokenTypes::PRINTFN, token_value: String::from(i)});
                }
            }

            //misc
            else if i == "class"{
                res.push(Token{ token_type: TokenTypes::CLASS, token_value: String::from(i)});
            }else if i == "NewLine"{
                res.push(Token{ token_type: TokenTypes::NEWLINE, token_value: String::from("NewLine")});
            }

            //operators
            else if i == "+"{
                res.push(Token{ token_type: TokenTypes::PLUS, token_value: String::from(i)});
            }else if i == "-" {
                res.push(Token{ token_type: TokenTypes::MINUS, token_value: String::from(i)});
            }else if i == "/" {
                res.push(Token{ token_type: TokenTypes::DIV, token_value: String::from(i)});
            }else if i == "*" {
                res.push(Token{ token_type: TokenTypes::MUL, token_value: String::from(i)});
            }else if i == "="{
                res.push(Token{ token_type: TokenTypes::EQUALS, token_value: String::from(i)});
            }

            //statments
            else if i == "=="{
                res.push(Token{ token_type: TokenTypes::IFEQUALS, token_value: String::from(i)});
            }else if i == "<="{
                res.push(Token{ token_type: TokenTypes::LESSOREQUAL, token_value: String::from(i)});
            }else if i == "=>"{
                res.push(Token{ token_type: TokenTypes::MOREOREQUAL, token_value: String::from(i)});
            }else if i == "<"{
                res.push(Token{ token_type: TokenTypes::SMALLERTHAN, token_value: String::from(i)});
            }else if i == ">"{
                res.push(Token{ token_type: TokenTypes::BIGGERTHAN, token_value: String::from(i)});
            }else if i == "!=" || i == "not"{
                res.push(Token{ token_type: TokenTypes::NOT, token_value: String::from(i)});
            }else if i == "&&" || i == "and"{
                res.push(Token{ token_type: TokenTypes::AND, token_value: String::from(i)});
            }else if i == "||" || i == "or"{
                res.push(Token{ token_type: TokenTypes::OR, token_value: String::from(i)});
            }

            //loops
            else if i == "for"{
                res.push(Token{ token_type: TokenTypes::FOR, token_value: String::from(i)});
            }else if i == "while"{
                res.push(Token{ token_type: TokenTypes::WHILE, token_value: String::from(i)});
            }else if i == "if"{
                res.push(Token{ token_type: TokenTypes::IF, token_value: String::from(i)});
            }else if i == "action"{
                res.push(Token{ token_type: TokenTypes::ACTION, token_value: String::from(i)});
            }else if i == "then"{
                res.push(Token{ token_type: TokenTypes::THEN, token_value: String::from(i)});
            }else if i[..1] == String::from("&"){
                res.push(Token{ token_type: TokenTypes::COMMENT, token_value: String::from(i)});
            }

            else{
                res.push(Token{ token_type: TokenTypes::IDENTIFIER, token_value: String::from(i)});
            }
        }
        
        return res;
    }
}

//parser
struct Parser {
    inp: Vec<Token>
}

trait Pars{
    fn parse(self);
}

//datatypes
#[derive(PartialEq)]
#[derive(Debug)]
pub enum DataTypes{
    ARRAY,
    STRING,
    TABLE,
    INTEGER,
    BOOL,
    DOUBLE,
    FLOAT,
}

impl fmt::Display for DataTypes{
    fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            DataTypes::FLOAT => write!(f,"FLOAT"),
            DataTypes::DOUBLE => write!(f,"DOUBLE"), 
            DataTypes::STRING => write!(f,"STRING"),
            DataTypes::INTEGER => write!(f,"INTEGER"),
            DataTypes::BOOL => write!(f,"BOOL"),
            DataTypes::ARRAY => write!(f,"ARRAY"),
            DataTypes::TABLE => write!(f,"TABLE"),
        }
    }
}
impl Pars for Parser{
    fn parse(self){
        //variable detection
        let mut varlist: Vec<Variable> = Vec::new();
        for i in 0..self.inp.len(){
            let cur = &self.inp[i];
            if cur.token_type.to_string() == DataTypes::STRING.to_string(){
                let mut list: Vec<&String> = Vec::new(); 
                for b in i..self.inp.len(){
                    let varcur = &self.inp[b];
                    if varcur.token_value == "NewLine"{
                        break;
                    }else{
                        list.push(&varcur.token_value);
                    }
                }
                varlist.push(Variable{ vartype:DataTypes::STRING, varname:  })
            } 
        }
        for b in varlist{println!("{}",b.to_string())}
    }
}

//utilities

#[derive(Debug)]
struct Variable{
    vartype: DataTypes,
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