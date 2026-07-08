use std::fs;
use std::process::exit;
mod parser;
use parser::*;
mod generator;
use generator::*;

// TODO CONSIDER THEM TYPES AND USE THEM AS LIKE UNSIGNED(BOX<VarType>)
// #[derive(Debug, Clone)]
// enum TypeComplement{
// 	Unsigned,
// 	Const,
// 	// Short,
// 	Long,
// }

#[derive(Debug, Clone)]
enum VarType{
	Int,
	Bool,
	Float,
	Double,
	Char,
	Void,
	Pointer(Box<VarType>),
	Array(Box<VarType>, usize),
	Struct(String),
	Variadic,
	Long(Box<VarType>),
	Short(Box<VarType>),
	Const(Box<VarType>),
	Unsigned(Box<VarType>),
	Unknown,
	// String,
}

#[derive(Debug, Clone)]
enum TokenType{
	Name(String),
	Typedef,
	Enum,
	Struct,
	Bool(bool),
	Assign,
	OBrace,
	CBrace,
	OParen,
	CParen,
	OBrack,
	CBrack,
	Comma,
	Number(usize),
	Hex(String), // fuck it we store hexadecimal numbers as strings
	Not,
	Semicolon,
	Star,
	Minus,
	Plus,
	Divide,
	Type(VarType),
	// TypeComplement(TypeComplement),
	String(String),
	Char(u8),
}


fn assign_type(token: impl AsRef<str>)->TokenType{
	let token_str = token.as_ref();
	match token_str {
		"typedef"=>TokenType::Typedef,
		"enum"=>TokenType::Enum,
		"struct"=>TokenType::Struct,
		"{"=>TokenType::OBrace,
		"}"=>TokenType::CBrace,
		"("=>TokenType::OParen,
		")"=>TokenType::CParen,
		"["=>TokenType::OBrack,
		"]"=>TokenType::CBrack,
		"true"=>TokenType::Bool(true),
		"false"=>TokenType::Bool(false),
		"="=>TokenType::Assign,
		","=>TokenType::Comma,
		"!"=>TokenType::Not,
		";"=>TokenType::Semicolon,
		"void"=>TokenType::Type(VarType::Void),
		"float"=>TokenType::Type(VarType::Float),
		"int"=>TokenType::Type(VarType::Int),
		"char"=>TokenType::Type(VarType::Char),
		"double"=>TokenType::Type(VarType::Double),
		"bool"=>TokenType::Type(VarType::Bool),
		"short"=>TokenType::Type(VarType::Short(Box::new(VarType::Unknown))),
		"long"=>TokenType::Type(VarType::Long(Box::new(VarType::Unknown))),
		"unsigned"=>TokenType::Type(VarType::Unsigned(Box::new(VarType::Unknown))),
		"const"=>TokenType::Type(VarType::Const(Box::new(VarType::Unknown))),
		"..."=>TokenType::Type(VarType::Variadic),
		"*"=>TokenType::Star,
		"-"=>TokenType::Minus,
		"+"=>TokenType::Plus,
		"/"=>TokenType::Divide,
		_=>{
			if token_str.chars().nth(0).unwrap() == '\''{
				if token_str.chars().last().unwrap() == '\''{
					TokenType::Char(token_str.as_bytes()[0])
				}
				else{println!("Unclosed char");exit(-1);}
			}else if token_str.chars().last().unwrap() == '\"'{
				if token_str.chars().last().unwrap() == '\"'{
					TokenType::String(token_str.to_string())
				}
				else{println!("Unclosed string");exit(-1);}
			} 
			else{
				let n = token_str.parse::<usize>();
				if n.is_ok(){
					TokenType::Number(n.unwrap())
				}else{
					if token_str.chars().nth(0).unwrap() == '0' && token_str.chars().nth(1).unwrap() == 'x'{
						TokenType::Hex(token_str.to_string())
					}else{
						TokenType::Name(token_str.to_string())
					}
				}
			}
		},
	}
}

fn main(){
	let content = fs::read_to_string("raylib.h").expect("Can't read the file");
	let mut peekable_data = content.chars().peekable();
	
	// remove comments
	let mut continue_reading = true;
	let mut single_line_comment = false;
	let mut multi_line_comment = false;
	let mut clean_data = String::new();
	while continue_reading{
		match peekable_data.next(){
			Some(chara)=>{
				if single_line_comment {
					while peekable_data.next() != Some('\n'){
						continue;
					}
					single_line_comment = false;
					continue;
				}
				if multi_line_comment{
					loop {
						let next_data = peekable_data.next();
						if next_data == Some('*'){
							if peekable_data.peek() == Some(&'/'){
								peekable_data.next();
								multi_line_comment = false;
								break;
							}
						}
						if next_data == Some('\n'){
						}
					}
					continue;
				}
				if chara == '/' {
					match peekable_data.peek(){
						Some('/') => {single_line_comment = true;continue}
						Some('*') => {multi_line_comment = true;continue;}
						_=>{}
					}
				}
				// treat preprocessor as single commnents to ignore for now
				if chara == '#'{
					single_line_comment = true;continue
				}
				clean_data.push_str(&String::from(chara));
			},
			None=>{
				continue_reading = false;
			}

		}
	}

	// split by space
	// let split_content: Vec<&str> = clean_data.split(" ").collect();
	// let mut new_words = Vec::new();
	let mut tokens = Vec::new();
    let mut current = String::new();
    let mut is_in_str_or_char = false;

    for c in clean_data.chars() {
        match c {
        	'\'' | '\"' => {
        		current.push(c);
				is_in_str_or_char = !is_in_str_or_char;
				continue;
        	}
            ' ' | '\t' | '\n' => {
            	if !is_in_str_or_char{
	                if !current.is_empty() {
	                    tokens.push(assign_type(std::mem::take(&mut current)));
	                }
            	}else{current.push(c)}
            }
            '(' | ')' | '{' | '}' | '[' | ']' | ';' | '*' | '!' | ',' => {
            	if !is_in_str_or_char{
	                if !current.is_empty() {
	                    tokens.push(assign_type(std::mem::take(&mut current)));
	                }
	                tokens.push(assign_type(c.to_string()));
            	}else{current.push(c)}
            }
            _ => current.push(c),
        }
    }

    if !current.is_empty() {
        tokens.push(assign_type(current));
    }

	// println!("{:?}", tokens);
    let ast = parse(tokens);
    let file_vec = generate(&ast);
    let mut file = String::new();
    for line in file_vec{
    	file.push_str(&line);
    	file.push_str("\n");
    }
    std::fs::write("raylib.rs", file);

	// let tokens = Vec::new();
	// for word in split_content{
		// if word.trim() != ""{
	// let tokens = split_regex.find_iter(clean_data).map(|m| m.as_str()).collect();
			// let separated_words = word.trim().split(";").split("(").split(")").split("{").split("}").split(",").split("=").split("*");
			// for word in separated_words}{
			// new_words.push();
		// }
	// }

}