#[allow(dead_code)]
use std::env;
use std::fs;
use colored::*;
use std::fmt;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "help" {
        println!("Usage:Fuel <run or compile> <filepath>\n");
    }else{
    if args.len() < 3 {
        println!("{}","Please provide a filepath".red()); 
    }else if args.len() > 3{
        println!("{}","Too many arguments".red());
    }
    else{
        if args[1] == "run"{
            println!("{} code runing!\n","unoptimized".green());
            let lexer = Lexer{input: read_file(&args[2])};
            lexer.lex();
        }else if args[1] == "compile"{
            println!("building");
        }
    }
}
}

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

fn read_file(filepath: &String) -> String{
    let contents = fs::read_to_string(filepath)
        .expect("Something went wrong reading the file");
    return contents;
}

struct Token{
    token_type: TokenTypes,
    token_value: String,
}

struct Lexer{
    input: String,
}

trait LEXER{
    fn lex(&self) -> Vec<Token>;
}

impl LEXER for Lexer{
    fn lex(&self) -> Vec<Token>{
        let mut res: Vec<Token> = Vec::new();
        let spl = self.input.split_whitespace();
        for i in spl{
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
            }
            //functions
            else if i == "input"{
                res.push(Token{ token_type: TokenTypes::INPUT, token_value: String::from(i)});
            }else if i == "print"{
                res.push(Token{ token_type: TokenTypes::PRINTFN, token_value: String::from(i)});
            }
            else{res.push(Token{ token_type: TokenTypes::IDENTIFIER, token_value: String::from(i)});}
        }
        return res;
    }
}