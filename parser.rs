use std::collections::HashSet;
use crate::{TokenType, VarType};
use std::process::exit;

struct ParsingContext{
	token_index: usize,
	typedef_structs: HashSet<String>,
	is_a_function_call: bool,
	ast: AST,
}

#[derive(Debug, PartialEq)]
enum TokenKind{
	Name,
	Typedef,
	Enum,
	Struct,
	Bool,
	Assign,
	OBrace,
	CBrace,
	OParen,
	CParen,
	OBrack,
	CBrack,
	Comma,
	Number,
	Hex,
	Not,
	Semicolon,
	Star,
	Minus,
	Plus,
	Divide,
	Type,
	// TypeComplement,
	String,
	Char,
}

impl TokenType{
	fn kind(&self)->TokenKind{
		match self{
			TokenType::Name(_) => TokenKind::Name,
			TokenType::Typedef => TokenKind::Typedef,
			TokenType::Enum => TokenKind::Enum,
			TokenType::Struct => TokenKind::Struct,
			TokenType::Bool(_) => TokenKind::Bool,
			TokenType::Assign => TokenKind::Assign,
			TokenType::OBrace => TokenKind::OBrace,
			TokenType::CBrace => TokenKind::CBrace,
			TokenType::OParen => TokenKind::OParen,
			TokenType::CParen => TokenKind::CParen,
			TokenType::OBrack => TokenKind::OBrack,
			TokenType::CBrack => TokenKind::CBrack,
			TokenType::Comma => TokenKind::Comma,
			TokenType::Number(_) => TokenKind::Number,
			TokenType::Hex(_) => TokenKind::Hex,
			TokenType::Not => TokenKind::Not,
			TokenType::Semicolon => TokenKind::Semicolon,
			TokenType::Star => TokenKind::Star,
			TokenType::Minus => TokenKind::Minus,
			TokenType::Plus => TokenKind::Plus,
			TokenType::Divide => TokenKind::Divide,
			TokenType::Type(_) => TokenKind::Type,
			// TokenType::TypeComplement(_) => TokenKind::TypeComplement,
			TokenType::String(_) => TokenKind::String,
			TokenType::Char(_) => TokenKind::Char,
		}
	}
}

#[derive(Debug, Clone)]
pub enum Statement{
	Struct(String, Vec<Statement>),
	Enum(String, Vec<(String, Option<TokenType>)>), // name of enum + vec of pair of name of field and optionally a value
	Variable(VarType, String),
	TypeAlias(VarType, String),
	OpaqueStruct(String, String),
	MultiDeclaration(VarType, Vec<String>),
	Function(VarType, String, Vec<Statement>),
	Void,
	Variadic,
	Unknown,
}

#[derive(Debug)]
pub struct AST{
	pub signatures: Vec<Statement>,
}

// fn get_token_index(parsing_context: &ParsingContext)->usize{
// 	unsafe{TOKEN_INDEX}
// }

fn update_token_index(parsing_context: &mut ParsingContext)->usize{
	parsing_context.token_index+=1;parsing_context.token_index
}

fn token_peek(tokens : &Vec<TokenType>, parsing_context: &mut ParsingContext)->TokenType{
	tokens[parsing_context.token_index+1].clone()
}

fn expect<'a>(tokens: &'a Vec<TokenType>, token_type: TokenKind, parsing_context: &mut ParsingContext)->&'a TokenType{
	let token = &tokens[parsing_context.token_index];
	update_token_index(parsing_context);
	if token.kind() == token_type{
		token
	}else{abort_parsing(format!("Expected {:?}, got {:?}", token_type, token), parsing_context);unreachable!()}
}

pub fn parse(mut tokens: Vec<TokenType>)->AST{
	let mut parsing_context = ParsingContext{token_index: 0, typedef_structs: HashSet::new(), is_a_function_call: false, ast: AST{signatures:Vec::new()}};
	// for statement in files do that :
	while !(parsing_context.token_index >= tokens.len()){
		let signature = parse_statements(&mut tokens, &mut parsing_context);
    	parsing_context.ast.signatures.push(signature);
    }
    println!("{:?}", parsing_context.ast);
    println!("finished parsing");
    return parsing_context.ast;
}

fn extract_name(name: &TokenType, parsing_context: &mut ParsingContext)->String{
	match name{
		TokenType::Name(extr_name)=>{
			extr_name.clone()
		}
		_=>{abort_parsing("Expected a name to extract content", parsing_context);unreachable!();}
	}
}

fn extract_type(var_type: &TokenType, parsing_context: &mut ParsingContext)->VarType{
	match var_type{
		TokenType::Type(extr_type)=>{
			extr_type.clone()
		}
		_=>{abort_parsing("Expected a type to extract content", parsing_context);unreachable!();}
	}
}

fn abort_parsing(reason: impl AsRef<str>, parsing_context: &mut ParsingContext){
	println!("ast : {:?}", parsing_context.ast);
	println!("Aborting parsing : {}", reason.as_ref());
	exit(-1);
}

// unsigned short int name;
fn resolve_type_complement(tokens: &mut Vec<TokenType>, parsing_context: &mut ParsingContext)->VarType{
	// if extract_name(&tokens[get_token_index()], ast) == String::from("GetFileModTime"){
	// unsafe{std::arch::asm!("int3");}
	// }
	match &tokens[parsing_context.token_index]{
		TokenType::Name(original_name)=>{
			update_token_index(parsing_context); 
			let mut found_type = String::new();
			if parsing_context.typedef_structs.contains(original_name){
				found_type = original_name.to_string();
			}
			else{abort_parsing(format!("expected a custom type, got {} instead", original_name), parsing_context);}
			// TODO is also may be another typedef : 
			// typedef a b
			// typedef b c <-
			let mut original_type = VarType::Struct(original_name.clone());
			if tokens[parsing_context.token_index].kind() == TokenKind::Star{
				original_type = VarType::Pointer(Box::new(original_type));
				update_token_index(parsing_context);
			}
			return original_type;
		},
		TokenType::Type(var_type)=>{
			update_token_index(parsing_context);
			match var_type{
				VarType::Short(_)=>{
					if tokens[parsing_context.token_index].kind() != TokenKind::Type{
						return VarType::Short(Box::new(VarType::Int));
					}else{
						return VarType::Short(Box::new(resolve_type_complement(tokens, parsing_context)));
					}
				},
				VarType::Long(_)=>{
					if tokens[parsing_context.token_index].kind() != TokenKind::Type{
						return VarType::Long(Box::new(VarType::Int));
					}else{
						return VarType::Long(Box::new(resolve_type_complement(tokens, parsing_context)));
					}
				},
				VarType::Unsigned(_)=>{
					if tokens[parsing_context.token_index].kind() != TokenKind::Type{
						return VarType::Unsigned(Box::new(VarType::Int));
					}else{
						return VarType::Unsigned(Box::new(resolve_type_complement(tokens, parsing_context)));
					}
				},
				VarType::Const(_)=>{return VarType::Const(Box::new(resolve_type_complement(tokens, parsing_context)));},
				_=>{return var_type.clone();}
			}
		}
		_=>{abort_parsing(format!("Unexpected token in compound type : {:?}", &tokens[parsing_context.token_index]), parsing_context);unreachable!();}
	}
}

fn parse_statements(tokens: &mut Vec<TokenType>, parsing_context: &mut ParsingContext) -> Statement{
	let mut token = &tokens[parsing_context.token_index].clone();
	match token {
		TokenType::Typedef=>{
			let token = &tokens[update_token_index(parsing_context)];update_token_index(parsing_context);
			match token{
				TokenType::Struct=>{
					let name = expect(&tokens, TokenKind::Name, parsing_context).clone();
					if tokens[parsing_context.token_index].kind() != TokenKind::OBrace{
						// println!("Opaque struct definition, TODO");
						if name.kind() != TokenKind::Name{abort_parsing(format!("Expected an opaque type name, got {:?}", tokens[parsing_context.token_index]), parsing_context);}
						let original_name = tokens[parsing_context.token_index].clone();
						let new_name = expect(&tokens, TokenKind::Name, parsing_context).clone();
						expect(&tokens, TokenKind::Semicolon, parsing_context);
						println!("newname : {:?}, original_name : {:?}", new_name, name);
						let extr_name = extract_name(&new_name, parsing_context);
						let extr_original_name = extract_name(&name, parsing_context);
						parsing_context.typedef_structs.insert(extr_original_name.clone());
						parsing_context.typedef_structs.insert(extr_name.clone());
						return Statement::OpaqueStruct(extr_original_name, extr_name);
					}
					expect(&tokens, TokenKind::OBrace, parsing_context);
					let mut declarations = Vec::new();
					while tokens[parsing_context.token_index].kind() != TokenKind::CBrace{
						declarations.push(parse_statements(tokens, parsing_context));
					}
					expect(&tokens, TokenKind::CBrace, parsing_context);
					expect(&tokens, TokenKind::Name, parsing_context); // name again
					expect(&tokens, TokenKind::Semicolon, parsing_context);
					// update_token_index();
					println!("found struct name : {:?}", name);
					let extr_name = extract_name(&name, parsing_context);
					parsing_context.typedef_structs.insert(extr_name);
					Statement::Struct(extract_name(&name, parsing_context), declarations)
				},
				TokenType::Enum=>{
					expect(&tokens, TokenKind::OBrace, parsing_context);
					let mut declarations = Vec::new();
					loop{
						let name = expect(&tokens, TokenKind::Name, parsing_context);
						let mut val = None;
						if tokens[parsing_context.token_index].kind() == TokenKind::Assign{
							update_token_index(parsing_context);
							if tokens[parsing_context.token_index].kind() == TokenKind::Number ||
							tokens[parsing_context.token_index].kind() == TokenKind::Hex{
								val = Some(tokens[parsing_context.token_index].to_owned());
								update_token_index(parsing_context);
							}
						}
						if tokens[parsing_context.token_index].kind() == TokenKind::CBrace{update_token_index(parsing_context);break;}
						expect(&tokens, TokenKind::Comma, parsing_context);
						if tokens[parsing_context.token_index].kind() == TokenKind::CBrace{update_token_index(parsing_context);break;}
						declarations.push((extract_name(name, parsing_context), val));
					}
					let name = expect(&tokens, TokenKind::Name, parsing_context);
					expect(&tokens, TokenKind::Semicolon, parsing_context);
					println!("parsed enum {:?}", name);
					Statement::Enum(extract_name(name, parsing_context), declarations)
				},
				TokenType::Name(original_name)=>{
					// typedef name name
					let mut found_type = String::new();
					if parsing_context.typedef_structs.contains(original_name){
						found_type = original_name.to_string();
					}
					else{abort_parsing(format!("expected a custom type, got {} instead", original_name), parsing_context);}
					// TODO is also may be another typedef : 
					// typedef a b
					// typedef b c <-
					let mut original_type = VarType::Struct(original_name.clone());
					if tokens[parsing_context.token_index].kind() == TokenKind::Star{
						original_type = VarType::Pointer(Box::new(original_type));
						update_token_index(parsing_context);
					}

					let alias = expect(&tokens, TokenKind::Name, parsing_context);
					expect(&tokens, TokenKind::Semicolon, parsing_context);
					println!("found type alias : {:?}", alias);
					let extr_name = extract_name(alias, parsing_context);
					parsing_context.typedef_structs.insert(extr_name);
					Statement::TypeAlias(original_type, extract_name(alias, parsing_context))
				}
				_=>{abort_parsing("With typedef expected struct or enum or an alias", parsing_context);unreachable!();}
			}
			// println!("found typedef");
		}
		TokenType::Type(var_type)=>{
			let mut new_var_type = var_type.to_owned();
			match new_var_type{
				VarType::Unsigned(_) | VarType::Const(_) | VarType::Short(_) | VarType::Long(_)=>{
					new_var_type = resolve_type_complement(tokens, parsing_context);
					println!("resolved complement decl : {:?}", new_var_type);
				},
				_=>{update_token_index(parsing_context);}
			}
			if tokens[parsing_context.token_index].kind() == TokenKind::Star{
				new_var_type = VarType::Pointer(Box::new(new_var_type.clone()));
				update_token_index(parsing_context);
				while tokens[parsing_context.token_index].kind() == TokenKind::Star{
					new_var_type = VarType::Pointer(Box::new(new_var_type.clone()));
					update_token_index(parsing_context);
				}
			}
			match new_var_type{
				VarType::Void=>{if tokens[parsing_context.token_index].kind() == TokenKind::CParen{update_token_index(parsing_context); expect(&tokens, TokenKind::Semicolon, parsing_context); return Statement::Void;}}, VarType::Variadic=>{update_token_index(parsing_context); /*expect(&tokens, TokenKind::CParen);*/ return Statement::Variadic;}, _=>{}}
			println!("new var type : {:?}", new_var_type);
			let name = expect(&tokens, TokenKind::Name, parsing_context).clone();
			match &tokens[parsing_context.token_index]{
				TokenType::Semicolon=>{
					expect(&tokens, TokenKind::Semicolon, parsing_context);
					Statement::Variable(new_var_type.to_owned(), extract_name(&name, parsing_context))
				},
				TokenType::Comma | TokenType::CParen=>{
					if token_peek(&tokens, parsing_context).kind() == TokenKind::Type || (token_peek(&tokens, parsing_context).kind() == TokenKind::Name && parsing_context.is_a_function_call) || tokens[parsing_context.token_index].kind() == TokenKind::CParen{
						let mut original_type = new_var_type.to_owned();
						if token_peek(&tokens, parsing_context).kind() == TokenKind::Name{
							if let TokenType::Name(original_name) = token_peek(&tokens, parsing_context){
								let mut found_type = String::new();
								if parsing_context.typedef_structs.contains(&original_name){
									found_type = original_name;
								}
								else{abort_parsing(format!("expected a custom type, got {} instead", original_name), parsing_context);}
								
								// TODO is also may be another typedef : 
								// typedef a b
								// typedef b c <-
								let mut original_type = VarType::Struct(extract_name(&name, parsing_context));
								if tokens[parsing_context.token_index].kind() == TokenKind::Star{
									original_type = VarType::Pointer(Box::new(original_type));
									update_token_index(parsing_context);
								}
							}
						}
						println!("new arg type : {:?}, name : {:?}", new_var_type, name);
						update_token_index(parsing_context);
						Statement::Variable(original_type, extract_name(&name, parsing_context))
					}else{
						let mut names = Vec::new();
						names.push(extract_name(&name, parsing_context));
						// update_token_index();
						while tokens[parsing_context.token_index].kind() != TokenKind::Semicolon{
							update_token_index(parsing_context);
							let name = expect(&tokens, TokenKind::Name, parsing_context);
							names.push(extract_name(name, parsing_context));
							// update_token_index();
						}
						expect(&tokens, TokenKind::Semicolon, parsing_context);
						Statement::MultiDeclaration(new_var_type.to_owned(), names)
					}
				},
				TokenType::OBrack=>{
					update_token_index(parsing_context);
					let size = expect(&tokens, TokenKind::Number, parsing_context);
					if let TokenType::Number(num) = size{
						expect(&tokens, TokenKind::CBrack, parsing_context);
						expect(&tokens, TokenKind::Semicolon, parsing_context);
						return Statement::Variable(VarType::Array(Box::new(new_var_type.to_owned()), *num), extract_name(&name, parsing_context));
					}
					unreachable!();
				},
				TokenType::OParen=>{
					update_token_index(parsing_context);
					// statement
					let mut args = Vec::new();
					parsing_context.is_a_function_call = true;
					loop{
						let statement = parse_statements(tokens, parsing_context);
						println!("added arg statement : {:?}", statement);
						if let Statement::Void = statement{break;}
						args.push(statement);
						if tokens[parsing_context.token_index].kind() == TokenKind::Semicolon{update_token_index(parsing_context); break;}/*else{expect(&tokens, TokenKind::Comma);}*/
					}
					// if extract_name(&name) == String::from("SetShaderValueV"){
					// 	unsafe{std::arch::asm!("int3");}
					// }
					// expect(&tokens, TokenKind::Semicolon);
					println!("Parsed function {} ", extract_name(&name, parsing_context));
					// if extract_name(&name, ast) == String::from("GetFileLength"){
					// 	unsafe{std::arch::asm!("int3");}
					// }
					parsing_context.is_a_function_call = false;
					Statement::Function(new_var_type, extract_name(&name, parsing_context), args)

				}
				_=>{unimplemented!("parsing type statement {:?}", &tokens[parsing_context.token_index])}
			}
		}
		// TokenType::TypeComplement(complement)=>{
			

		// 	// if let Statement::Variable(var_type, _) = &mut declaration{
		// 	// 	match var_type{
		// 	// 		VarType::Char=>{match complement{TypeComplement::Unsigned=>{*var_type = VarType::UChar}, _=>{todo!("{:?}", complement)}}}
		// 	// 		VarType::Int=>{match complement{TypeComplement::Unsigned=>{*var_type = VarType::UInt}, _=>{todo!("{:?}", complement)}}}
		// 	// 		VarType::Pointer=>{match complement{TypeComplement::Unsigned=>{*var_type = VarType::Pointer()}, _=>{todo!("{:?}", complement)}}}
		// 	// 		_=>{todo!("{:?}", var_type)},
		// 	// 	}
		// 	// }
		// 	declaration

		// }
		TokenType::Name(name)=>{
			if name == "RLAPI"{
				println!("found function");
				update_token_index(parsing_context);
				return parse_statements(tokens, parsing_context);
			}
			unsafe{
					let mut found_type = String::new();
					if parsing_context.typedef_structs.contains(name){
						found_type = name.to_string();
					}
					else{abort_parsing(format!("expected a custom type, got {} instead", name), parsing_context);}
					
					// TODO is also may be another typedef : 
					// typedef a b
					// typedef b c <-
					tokens[parsing_context.token_index] = TokenType::Type(VarType::Struct(found_type.to_string()));
					return parse_statements(tokens, parsing_context);
			}
		}
		_=>{unimplemented!("parsing statement {:?}", token)}
	}
}