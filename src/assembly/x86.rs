use std::{collections::{HashMap, VecDeque}, fmt::{Display, self}};

use super::three_ac::Variable3ACType;

pub fn convert_x86(ir: String) -> String {
	let mut ir = ir.as_str().split("[BEGIN ").collect::<VecDeque<&str>>();
	ir.pop_front(); // get rid of some whitespace

	let table : &mut FnSymbolTableWrapper = &mut FnSymbolTableWrapper(HashMap::new());
	let mut output : Vec<String> = Vec::new();

	output.push(match ir.pop_front() { //globals
		Some(x) => table.translate_globals(x),
		None => "".to_string()
	});
	
	output.push("\n.text".to_string());

	for function in ir {
		output.push(table.translate_fn(function));
	}

	output.push("\n\tmovq $60,\t%rax\n\tmovq $1,\t%rdi\n\t\n\tsyscall".to_string());
	output.push("".to_string());
    output.join("\n")
}

struct SemanticSymbol {
	location: String,
	global: bool,
	sym_type: Variable3ACType
}

impl Display for SemanticSymbol {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.global {
			write!(f, "⬤ {}\t{}", self.location, self.sym_type)
		} else {
			write!(f, "○ {}\t{}", self.location, self.sym_type)
		}
    }
}

struct FnSymbolTableWrapper(HashMap<String, SemanticSymbol>);
trait FnSymbolTable : Display {
	// fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result;
	fn translate_globals(&mut self, globals: &str) -> String;
	fn populate_fn(&mut self, fn_vars: Vec<&str>) -> usize; //returns the num of locals
	fn translate_fn(&mut self, function: &str) -> String; 
	fn translate_fn_code(&mut self, function: &str, num_locals: usize) -> String; 

	fn translate_statement(&self, statement: &str, num_locals: usize) -> String;
	fn translate_assign(&self, statement: &str) -> String;
	fn translate_compare(&self, ops: Vec<&str>) -> String;
	fn translate_multdiv(&self, ops: Vec<&str>) -> String;
	fn translate_write(&self, statement: &str) -> String;
	fn translate_read(&self, statement: &str) -> String;
	fn translate_if(&self, statement: &str) -> String;

	fn clear_fn(&mut self);

	// for these next two, the first ret val is possible code that needs to be prepended to 
	// the full statement block for the MAGIC keyword and lib call
	
	// the second ret val is the actual location to reference 
	// (gbl location, %rbp offset, or %rdi in case of magic)
	fn get_loc(&self, label: &str) -> (&str, &str); 
	fn parse_loc(&self, label: &str) -> (&str, String); 
	fn get_type(&self, label: &str) -> Variable3ACType; 
}

impl Display for FnSymbolTableWrapper {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut output: Vec::<String> = Vec::new();
		for (key, value) in self.0.iter() {
			output.push(format!("{}:\t {}", key, value));
		}
        write!(f, "{}", output.join("\n"))
    }
}

impl FnSymbolTable for FnSymbolTableWrapper {
	fn translate_globals(&mut self, globals: &str) -> String {
		let mut output : Vec<String> = Vec::new();
		output.push(".globl main".to_string());
		let mut globals = globals.split('\n').collect::<VecDeque<&str>>();
		
		// whitespace byebye 
		globals.pop_front();

		// if front is blank, we have no globals
		let front = match globals.front() {
			Some(x) => *x,
			None => return output.join("\n")
		};
		if front.trim() == "" {
			return output.join("\n")
		}

		// get rid of whitespace/extra info we don't need to translate to x86 
		loop {
			let back = match globals.back() {
				Some(x) => *x,
				None => break
			};
			globals.pop_back();
			if back == "[END GLOBALS]" {
				break;
			} 
		}

		// we have globals, start processing
		output.push(".data".to_string());
		for global in globals {
			let label: String;
			let symbol: SemanticSymbol;

			if global.contains(' ') { // string 
				let (lbl, string) = match global.split_once(' ') {
					Some(x) => x,
					None => ("", "")
				};
				
				output.push(format!("{lbl}: .asciz {string}"));
				
				label = lbl.to_string();
				symbol = SemanticSymbol {
					location: format!("({lbl})"),
					global: true,
					sym_type: Variable3ACType::String
				};
			} else {// other kind of global
				let global : Vec<&str> = global.split("-").collect();
				label = global[0].to_string();
				let sym_type = match global[1] {
					"bool" => Variable3ACType::Bool,
					_ => Variable3ACType::Int
				};
				output.push(format!("gbl_{label}: .quad 0"));

				symbol = SemanticSymbol {
					location: format!("(gbl_{})", label),
					global: true,
					sym_type
				};
			}
			
			self.0.insert(format!("[{label}]"), symbol);
		}

		output.join("\n")
	}

	fn populate_fn(&mut self, fn_vars: Vec<&str>) -> usize {
		let mut num_locals = 0;
		for (i, var) in fn_vars.iter().enumerate() {
			let var = var.split('-').collect::<Vec<&str>>();
			let label = var[0];
			let sym_type = match var[1] {
				"bool" => Variable3ACType::Bool,
				_ => Variable3ACType::Int
			};
			let symbol = SemanticSymbol {
				location: format!("-{}(%rbp)", i*8 + 24),
				global: false,
				sym_type
			};
			self.0.insert(format!("[{label}]"), symbol);
			num_locals = i;
		}
		num_locals
	}

	fn translate_fn(&mut self, function: &str) -> String {
		let mut output : Vec<String> = Vec::new();
		let mut function = function.split("LOCALS]\n").collect::<Vec<&str>>().into_iter();
		
		function.next(); //extra info
		
		let locals = match function.next() {
			Some(x) => x,
			None => return "".to_string()
		};
		// println!("{}", &self);
		let num_locals : usize;
		if !locals.starts_with("\n[END ") {
			let mut locals = locals.split("\n").collect::<Vec<&str>>();
			locals.pop();
			num_locals = self.populate_fn(locals);
		} else {
			num_locals = 0;
		}

		// output.push(function_prologue(num_locals));
		let code = match function.next() {
			Some(x) => x,
			None => return "".to_string()
		};

		// println!("{}", locals);
		// println!("OWO!!!!!!!!!!!");
		// println!("{}", &self);
		// println!("{}", &code);
		output.push(self.translate_fn_code(code, num_locals));
		self.clear_fn();
		output.join("\n")
	}

	fn translate_fn_code(&mut self, code: &str, num_locals: usize) -> String {
		let mut output : Vec<String> = Vec::new();
		let mut code = code.split("\n").collect::<Vec<&str>>().into_iter();
		code.next();
		let function_name = match code.next() {
			Some(x) => x.split(":").collect::<Vec<&str>>()[0],
			None => return "".to_string()
		};
		output.push(format!("{}:{}", function_name, function_prologue(num_locals)));

		for stmt in code {
			output.push(self.translate_statement(stmt, num_locals));
		}

		output.join("\n")
	}
	
	fn translate_statement(&self, statement: &str, num_locals: usize) -> String {
		if statement.contains(":=") {
			return self.translate_assign(statement);
		}
		if statement.contains("nop") {
			return statement.to_string();
		}
		if statement.contains("leave") {
			return format!("{}:{}",
				statement.split(":").collect::<Vec<&str>>()[0],
				function_epilogue(num_locals));
		}
		if statement.contains("WRITE") {
			return self.translate_write(statement);
		}
		if statement.contains("READ") {
			return self.translate_read(statement);
		}
		if statement.contains("IFZ") {
			return self.translate_if(statement);
		}
		if statement.contains("goto") {
			let dest = statement.split(" ").collect::<Vec<&str>>()[1];
			return format!("\tjmp\t\t{dest}");
		}
		"".to_string()
	}

	fn translate_assign(&self, statement: &str) -> String {
		let ops = statement.split(' ').collect::<Vec<&str>>();
		let (_, dest) = self.get_loc(ops[0].trim());

		// direct assign, no math involved
		if ops.len() == 3 {
			let src = ops[2];
			match src.parse::<i32>() {
				Ok(x) => return format!("\tmovq ${},\t{}", x, dest),
				Err(_) => {
					let (prepend, src) = self.get_loc(src);
					return format!("{}\tmovq {},\t%rax\n\tmovq %rax,\t{}", prepend, src, dest)
				}
			};
		}

		// unary
		if ops.len() == 4 {
			let operand = get_operation_quad(ops[2]);
			let (prepend, src) = self.parse_loc(ops[3]);
			return format!("{}\tmovq {},\t%rax\n\t{} %rax\n\tmovq %rax,\t{}", prepend, src, operand, dest);
		}

		// binary
		match ops[3] {
			"EQ64" | "NEQ64" | "LT64" | "GT64" | "GTE64" | "LTE64" => return self.translate_compare(ops),
			"MULT64" | "DIV64" => return self.translate_multdiv(ops),
			_ => ()
		};
		
		let quad = get_operation_quad(ops[3]);
		let (prepend1, src1) = self.parse_loc(ops[2]);
		let (prepend2, src2) = self.parse_loc(ops[4]);
		let (prepend3, dest) = self.parse_loc(ops[0].trim());
		return format!(
			"{}{}\tmovq {},\t%rax\n\tmovq {},\t%rbx\n\t{} %rbx,\t%rax\n{}\tmovq %rax,\t{}", 
			prepend1, prepend2, src1, src2, quad, prepend3, dest
		)
	}

	fn translate_multdiv(&self, ops: Vec<&str>) -> String {
		let quad = get_operation_quad(ops[3]);
		let (prepend1, src1) = self.parse_loc(ops[2].trim());
		let (prepend2, src2) = self.parse_loc(ops[4].trim());
		let (prepend3, dest) = self.parse_loc(ops[0].trim());
		return format!(
			"{}{}\tmovq {},\t%rax\n\tmovq {},\t%rbx\n\t{}\t\t%rbx\n{}\tmovq %rax,\t{}", 
			prepend1, prepend2, src1, src2, quad, prepend3, dest
		)
	}

	fn translate_compare(&self, ops: Vec<&str>) -> String {
		let set_suffix = match ops[3] {
			"EQ64" => "e",
			"NEQ64" => "ne",
			"LT64" => "l",
			"GT64" => "g",
			"GTE64" => "ge",
			"LTE64" => "le",
			_ => ""
		};
		let (prepend1, src1) = self.parse_loc(ops[2]);
		let (prepend2, src2) = self.parse_loc(ops[4]);
		let (prepend3, dest) = self.parse_loc(ops[0].trim());
		return format!(
			"{}{}\tmovq {},\t%rax\n\tmovq {},\t%rbx\n\tset{}\t\t%al\n{}\tmovb %al,\t{}", 
			prepend1, prepend2, src1, src2, set_suffix, prepend3, dest
		)
	}

	fn translate_write(&self, statement: &str) -> String {
		let statement : Vec<&str> = statement.trim().split(" ").collect();
		let (prepend, loc) = self.parse_loc(statement[1]);
		let print_call = match self.get_type(statement[1]) {
			Variable3ACType::Bool => "printBool",
			Variable3ACType::Int => "printInt",
			Variable3ACType::String => "printString",
		};
		format!("{}\tmovq {},\t%rdi\n\tcallq {}", prepend, loc, print_call)
	}

	fn translate_read(&self, statement: &str) -> String {
		let statement : Vec<&str> = statement.trim().split(" ").collect();
		let (prepend, loc) = self.parse_loc(statement[1]);
		let print_call = match self.get_type(statement[1]) {
			Variable3ACType::Bool => "getBool",
			Variable3ACType::Int => "getInt",
			Variable3ACType::String => "getString",
		};
		format!("{}\tcallq {}\n\tmovq %rax,\t{}\n\t", prepend, loc, print_call)
	}

	fn translate_if(&self, statement: &str) -> String {
		let statement : Vec<&str> = statement.trim().split(" ").collect();
		let (prepend, predicate) = self.parse_loc(statement[1]);
		let dest = statement[3];		
		format!("{}\tmovb {},\t%al\n\tcmpb $0,\t%al\n\tje\t\t{}", prepend, predicate, dest)
	}

	fn clear_fn(&mut self) {
		self.0.retain(|_, sym| sym.global == false);
	}

	fn get_loc(&self, label: &str) -> (&str, &str) {
		match self.0.get(label) {
			Some(x) => ("", x.location.as_str()),
			None => {
				match label {
					"24Kmagic" => ("\n\tcallq magic\n", "%rdi"),
					"true" => ("", "$1"),
					"false" => ("", "$0"),
					_ => ("", "")
				}
			}
		}
	}
	
	fn parse_loc(&self, label: &str) -> (&str, String) {
		match label.parse::<i32>() {
			Ok(_) => {
				let label = format!("${label}").to_owned();
				("", label)
			},
			Err(_) => {
				let (a, b) = self.get_loc(label);
				(a, b.to_string())
			}
		}
	}
	fn get_type(&self, label: &str) -> Variable3ACType {
		match label {
			"true" | "false" | "24Kmagic" => Variable3ACType::Bool,
			_ => match label.parse::<i32>() {
				Ok(_) => Variable3ACType::Int,
				Err(_) => match self.0.get(label) {
					Some(x) => x.sym_type,
					None => Variable3ACType::Int               // TODO: does this actually get called?
				}
			}
		}
	}
}

fn function_prologue(num_locs: usize) -> String {
	format!("\tpushq %rbp\n\tmovq %rsp,\t%rbp\n\taddq $16,\t%rbp\n\tsubq ${},\t%rsp", num_locs*8)
}

fn function_epilogue(num_locs: usize) -> String {
	format!("\taddq ${},\t%rsp\n\tpopq %rbp\n\tretq", num_locs*8)
}

fn get_operation_quad(operation: &str) -> &str {
	match operation {
		"NEG64" => "notq",
		"ADD64" => "addq",
		"SUB64" => "subq",
		"MULT64" => "imulq",
		"DIV64" => "idivq",
		"AND64" => "andq",
		"OR64" => "orq",
		"NOT64" => "notq",
		&_ => ""
	}
}