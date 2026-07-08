use exit;
use parser::{AST, Statement};
use crate::{VarType, TokenType};

fn abort_generating(reason : impl AsRef<str>){
	println!("Aborting generation : {}", reason.as_ref());
	exit(-1);
}

struct RsType{
	prefix: String,
	size: usize,
	is_pointer: bool,
	is_constant: bool,
	is_char: bool,
	is_unsigned: bool,
	is_array: bool,
	arr_size: usize,
	requires_size: bool,
}

fn resolve_type(var_type: &VarType, rs_type: &mut RsType)->String{
	match var_type{
		VarType::Int=>{rs_type.requires_size=true;rs_type.prefix = "i".to_string();rs_type.size = 32;},
		VarType::Float=>{rs_type.requires_size=true;rs_type.prefix = "f".to_string();rs_type.size = 32;},
		VarType::Double=>{rs_type.requires_size=true;rs_type.prefix = "f".to_string();rs_type.size = 64;},
		VarType::Bool=>{rs_type.prefix = "bool".to_string()}
		VarType::Char=>{rs_type.requires_size=true; rs_type.is_char = true;rs_type.prefix = "i".to_string(); rs_type.size = 8},
		VarType::Void=>{rs_type.prefix = "c_void".to_string();}
		VarType::Pointer(inter_type)=>{resolve_type(inter_type, rs_type);if rs_type.is_char{if rs_type.is_unsigned{rs_type.prefix = "c_uchar".to_string()}else{rs_type.prefix = "c_char".to_string();};rs_type.requires_size=false;} rs_type.is_pointer = true},
		VarType::Struct(inter_type)=>{rs_type.prefix = inter_type.to_string();},
		VarType::Array(inter_type, size)=>{
			resolve_type(inter_type, rs_type);
			rs_type.is_array = true;
			rs_type.arr_size = *size;
		}
		VarType::Unsigned(inter_type)=>{
			resolve_type(inter_type, rs_type);
			rs_type.is_unsigned = true;
			rs_type.prefix = "u".to_string();
		}
		VarType::Const(inter_type)=>{
			resolve_type(inter_type, rs_type);
			rs_type.is_constant = true;
		}
		VarType::Short(inter_type)=>{
			resolve_type(inter_type, rs_type);
			rs_type.size = rs_type.size/2;
		},
		VarType::Long(inter_type)=>{
			resolve_type(inter_type, rs_type);
			rs_type.size = rs_type.size*2;
		}
		// VarType::Unsigned(intern_type)=>{
		// 	match &**intern_type{
		// 		VarType::Int=>"u32".to_string(),
		// 		VarType::Char=>"u8".to_string(),
		// 		VarType::Short(intern_type) | VarType::Long(intern_type){
		// 			resolve_type()
		// 		}
		// 		_=>{todo!("unsigned type resolver : {:?}", intern_type);}
		// 	}
		// }
		// VarType::Short(intern_type)=>{
		// 	match &**intern_type{
		// 		VarType::Int=>"i16".to_string(),
		// 		// VarType::Char=>"u8".to_string(),
		// 		_=>{todo!("short type resolver : {:?}", intern_type);}
		// 	}
		// }
		// VarType::Pointer(intern_type)=>{
		// 	let reslvd_intern_type = resolve_type(intern_type);
		// 	format!("*mut {}", reslvd_intern_type)
		// }
		// VarType::Struct(intern_type)=>{intern_type.to_string()},
		// VarType::Void=>"c_void".to_string(),
		_=>{todo!("type resolver : {:?}", var_type);}
	}
	let mut composed_type = String::new();
	if rs_type.is_array{composed_type.push_str("[");}
	if rs_type.is_pointer{
		if rs_type.is_constant{
			composed_type.push_str(&"*const ");
		}else{
			composed_type.push_str(&"*mut ");
		}
	}
	composed_type.push_str(&rs_type.prefix);
	if rs_type.requires_size{
		composed_type.push_str(&rs_type.size.to_string());
	}
	if rs_type.is_array{composed_type.push_str(&format!("; {}]", rs_type.arr_size));}
	composed_type.push_str(",");
	println!("{:?} got outputed as {}", var_type, composed_type);
	composed_type
}

fn generate_struct(name: impl AsRef<str>, fields: &Vec<Statement>, bindings: &mut Vec<String>, opaque: bool){
	let name_str = name.as_ref();
	bindings.push("#[repr(C)]".to_string());
	if !opaque{bindings.push("#[derive(Debug, Copy, Clone)]".to_string());}
	let mut publicity = String::new();
	if !opaque{publicity = "pub".to_string();}
	bindings.push(format!("pub struct {} {{", name_str));
	for field in fields{
		match field{
			Statement::Variable(var_type, var_name)=>{
				let mut fix_name = var_name.clone();
				if var_name == "type" || var_name == "box"{fix_name = format!("{}{}", "r#", var_name);}
				bindings.push(format!("    {} {}: {}", publicity, fix_name, resolve_type(var_type, &mut RsType{prefix: String::new(), size: 0, is_pointer: false, is_constant: false,	is_char: false, requires_size: false, is_unsigned: false, is_array: false, arr_size: 0})));
			},
			Statement::MultiDeclaration(var_type, name_vec)=>{
				let common_type = resolve_type(var_type, &mut RsType{prefix: String::new(), size: 0, is_pointer: false, is_constant: false,	is_char: false, requires_size: false, is_unsigned: false, is_array: false, arr_size: 0}).to_owned();
				for decl in name_vec{
					bindings.push(format!("    {} {}: {}", publicity, decl, common_type));
				}
			}
			_=>{todo!("{:?}", field)}
		}
		// bindings.push(format!("pub struct {} {{", name_str));
	}
	bindings.push("}".to_string());
}

fn generate_type_alias(original_type: &VarType, alias_name: impl AsRef<str>, bindings: &mut Vec<String>){
	match original_type{
		VarType::Struct(original_type_name)=>{
			bindings.push(format!("pub type {} = {};", alias_name.as_ref(), original_type_name));
		}VarType::Pointer(inter_type)=>{
			match &**inter_type{
				VarType::Struct(original_type_name)=>{
					bindings.push(format!("pub type {} = *mut {};", alias_name.as_ref(), original_type_name));
				}_=>{unreachable!();}
			}
		}
		_=>{todo!("more type alias : {:?}", original_type);}
	}
}

fn generate_enum(name: impl AsRef<str>, fields: &Vec<(String, Option<TokenType>)>, bindings: &mut Vec<String>){
	bindings.push(format!("enum {} {{", name.as_ref()));
	for field in fields{
		let (field_name, field_value) = field;
		match field_value{
			Some(value)=>{
				match value{
					TokenType::Hex(hex)=>{
						bindings.push(format!("    {} = {},", field_name, hex));
					}
					TokenType::Number(num)=>{
						bindings.push(format!("    {} = {},", field_name, num));
					}
					_=>{unreachable!("{:?}", value);}
				}
			}
			None=>{
				bindings.push(format!("    {},", field_name));
			}
		}
	}
	bindings.push("}".to_string());
}

fn generate_function(return_type: &VarType, fn_name: impl AsRef<str>, args: Vec<Statement>)->(String, String){
	let mut fn_sig = String::new();
	let mut wrapper_functions = String::new();
	let mut wrapped_function_unsafe = String::new();
	fn_sig.push_str(&format!("pub fn {}(", fn_name.as_ref()));
	wrapper_functions.push_str(&format!("pub fn {}(", fn_name.as_ref()));
	wrapped_function_unsafe.push_str(&format!("{}(", fn_name.as_ref()));
	for arg in &args{
		match arg{
			Statement::Variable(var_type, arg_name)=>{
				let mut var_type_str = resolve_type(&var_type, &mut RsType{prefix: String::new(), size: 0, is_pointer: false, is_constant: false,	is_char: false, requires_size: false, is_unsigned: false, is_array: false, arr_size: 0});
				let mut fix_name = arg_name.clone();
				if arg_name == "type" || arg_name == "box" {fix_name = format!("{}{}", "r#", arg_name);}
				fn_sig.push_str(&format!("{}: {} ", fix_name, var_type_str));
				if var_type_str == "*const c_char,"{var_type_str = "impl AsRef<str>,".to_string(); wrapped_function_unsafe.push_str(&format!("CString::new({}.as_ref()).unwrap().as_ptr(), ",fix_name))}else{
					wrapped_function_unsafe.push_str(&format!("{}, ",fix_name))
				}
				wrapper_functions.push_str(&format!("{}: {} ", fix_name, var_type_str));
			}Statement::Variadic=>{
				fn_sig.push_str("..., ");
			},
			_=>{unreachable!("{:?}", arg)}
		}
	}
	if args.len() >=1{
		fn_sig.pop();
		fn_sig.pop();
		wrapper_functions.pop();
		wrapper_functions.pop();
		wrapped_function_unsafe.pop();
		wrapped_function_unsafe.pop();
	}
	match return_type{
		VarType::Void=>{
			fn_sig.push_str(");");
			wrapper_functions.push_str("){");
		}
		_=>{
			fn_sig.push_str(&format!(")->{}", resolve_type(&return_type, &mut RsType{prefix: String::new(), size: 0, is_pointer: false, is_constant: false,	is_char: false, requires_size: false, is_unsigned: false, is_array: false, arr_size: 0})));
			fn_sig.pop();
			fn_sig.push_str(";");
			wrapper_functions.push_str(&format!(")->{}", resolve_type(&return_type, &mut RsType{prefix: String::new(), size: 0, is_pointer: false, is_constant: false,	is_char: false, requires_size: false, is_unsigned: false, is_array: false, arr_size: 0})));
			wrapper_functions.pop();
			wrapper_functions.push_str("{");
		}
	}
	wrapper_functions.push_str("unsafe{raw::");
	wrapper_functions.push_str(&wrapped_function_unsafe);
	wrapper_functions.push_str(")");
	wrapper_functions.push_str("}");
	wrapper_functions.push_str("}");
	(fn_sig, wrapper_functions)

	// pub fn InitWindow(width: i32, height: i32, title: *const c_char);
}

fn file_dbg_print(bindings: &Vec<String>){
	for line in bindings{
		println!("{}", line);
	}
}

pub fn generate(ast: &AST)->Vec<String>{
	let mut bindings: Vec<String> = Vec::new();
	bindings.push("use std::ffi::{c_char, c_uchar, c_void};".to_string());
	bindings.push("use std::ffi::CString;".to_string());
	let mut raw_functions: Vec<String> = Vec::new();
	let mut wrapper_functions: Vec<String> = Vec::new();
	raw_functions.push("mod raw{".to_string());
	raw_functions.push("    use ffi_bind::*;".to_string()); // file name of ffi implementations
	raw_functions.push("    unsafe extern \"C\" {".to_string());

	let mut i = 0;
	while !(i >= ast.signatures.len()){
		let statement = &ast.signatures[i];
		match statement{
			Statement::Struct(name, fields)=>{generate_struct(name, &fields, &mut bindings, false);},
			Statement::OpaqueStruct(opaque_name, alias_name)=>{
				generate_struct(opaque_name, &vec![Statement::Variable(VarType::Array(Box::new(VarType::Unsigned(Box::new(VarType::Short(Box::new(VarType::Short(Box::new(VarType::Int))))))), 0), "_unused".to_string())], &mut bindings, true);
				if opaque_name != alias_name{
					generate_type_alias(&VarType::Struct(opaque_name.to_string()), alias_name, &mut bindings);
				}
			}
			Statement::TypeAlias(original_type, alias_name)=>{generate_type_alias(&original_type, alias_name, &mut bindings)},
			Statement::Enum(name, fields)=>{generate_enum(name, fields, &mut bindings);}
			Statement::Function(return_type, fn_name, args)=>{
				let (raw_function_decl, wrapper_function_decl) = generate_function(return_type, fn_name, args.clone());
				raw_functions.push(format!("        {}",raw_function_decl));
				wrapper_functions.push(wrapper_function_decl);
			}
			_=>{file_dbg_print(&bindings);todo!("statment generator : {:?}", statement);},
		}
		i+=1;
	}
	if raw_functions.len() >= 1{
		raw_functions.push("    }}".to_string());
		for fun in raw_functions{
			bindings.push(fun);
		}
		for wrapper in wrapper_functions{
			bindings.push(wrapper);
		}
	}
	file_dbg_print(&bindings);
	bindings
}