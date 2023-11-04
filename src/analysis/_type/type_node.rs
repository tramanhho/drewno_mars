use super::*;
use super::TypeAnalyzer;
use super::ErrorType::*;
use std::cmp::min;
pub trait TypeAnalysisNode {
    fn analyze_type(&mut self, analyzer: &mut TypeAnalyzer);
}

pub trait TypeAnalysisStmtNode {
    fn analyze_type(&mut self, analyzer: &mut TypeAnalyzer, return_type: TypeKind);
}

pub trait EvaluateExpType {
    fn eval_type(&mut self, analyzer: &mut TypeAnalyzer) -> Result<Type, ()>;
}


impl TypeAnalysisNode for Program {
	fn analyze_type(&mut self, analyzer: &mut TypeAnalyzer) {
		for gbl in self.globals.iter_mut() {
			gbl.analyze_type(analyzer);
		}
	}
}

impl TypeAnalysisNode for Decl {
	fn analyze_type(&mut self, analyzer: &mut TypeAnalyzer) {
		use Decl::*;
		match self {
            VarDecl(x) => x.analyze_type(analyzer),
            ClassDecl(x) => x.analyze_type(analyzer),
            FnDecl(x) => x.analyze_type(analyzer),
        };
	}
}

//TODO: this
impl TypeAnalysisNode for ClassDecl {
	fn analyze_type(&mut self, analyzer: &mut TypeAnalyzer) {	
		analyzer.add_class(self);
	}
}

//TODO: this
impl TypeAnalysisNode for FnDecl {
	fn analyze_type(&mut self, analyzer: &mut TypeAnalyzer) {
		analyzer.add_fn(self);

		for stmt in self.body.iter_mut() {
			stmt.analyze_type(analyzer, *self.ret.kind.clone());
		}
	}
}

//TODO: this
impl TypeAnalysisNode for VarDecl {
	fn analyze_type(&mut self, analyzer: &mut TypeAnalyzer) {
		let init = match self.init_val {
			Some(ref mut x) => { x.analyze_type(analyzer); x },
			None => return,
		};
		match &init.expr_type {
			Some(expr_type) => {
				if *expr_type != self.var_type {
					analyzer.report_error(&BadAssign, &init.span);
				}
			},
			None => ()
		}
	}
}

impl TypeAnalysisStmtNode for Stmt {
	fn analyze_type(&mut self, analyzer: &mut TypeAnalyzer, return_type: TypeKind) {
		use Stmt::*;

		match self {
			Block(x) => x.analyze_type(analyzer, return_type),
			Line(x) => x.analyze_type(analyzer, return_type),
			VarDecl(x) => x.analyze_type(analyzer),
		}
	}
}

impl TypeAnalysisStmtNode for BlockStmt {
	fn analyze_type(&mut self, analyzer: &mut TypeAnalyzer, return_type: TypeKind) {
		
	}
}

impl TypeAnalysisStmtNode for LineStmt {
	fn analyze_type(&mut self, analyzer: &mut TypeAnalyzer, parent_fn_return_type: TypeKind) {
		let mut variation = *self.kind.clone();

		use LineStmtKind::*;
		use TypeKind::*;
		use ExpKind::*;
		match variation {
			Assign { ref mut dest, ref mut src } => {
				let dest_type = dest.eval_type(analyzer);
				src.analyze_type(analyzer);
				let src_type = &src.expr_type;
				if dest_type.is_err() || src_type.is_none() {
					return;
				}
				//TODO: do the rest of the assign
			},
			PostDec{ ref mut loc } | PostInc{ ref mut loc} => {
				let loc_type = loc.eval_type(analyzer);
				if loc_type.is_err() {
					return;
				}

				if *loc_type.unwrap().kind != Prim(PrimType::Int) {
					analyzer.report_error(&WrongOpMath, &loc.span);
				}

			},
			Give { ref mut output} => {
				output.analyze_type(analyzer);
				match *output.kind.clone() {
					Loc(loc) => {
						if analyzer.has_fn(&loc.to_string()) {
							analyzer.report_error(&GiveFn, &output.span);
						}
						if analyzer.has_class(&loc.to_string()) {
							analyzer.report_error(&GiveClass, &output.span);
						}
					},
					CallExp(_) => {
						match output.expr_type.clone() {
							Some(return_type) => {
								if *return_type.kind == Prim(PrimType::Void) {
									analyzer.report_error(&GiveVoid, &output.span);
								}
							},
							None => ()
						}
					},
					_ => ()
				}
			},
			Take { ref mut recipient } => {
				if analyzer.has_fn(&recipient.to_string()) {
					analyzer.report_error(&ReadFn, &recipient.span);
				}
				if analyzer.has_class(&recipient.to_string()) {
					analyzer.report_error(&ReadClass, &recipient.span);
				}
			},
			Return { ref mut result} => {
				match result {
					Some(return_expr) => {
						return_expr.analyze_type(analyzer);
						match return_expr.expr_type.clone() {
							Some(actual_return_type) => {
								if *actual_return_type.kind == Prim(PrimType::Void) && parent_fn_return_type != Prim(PrimType::Void) {
									analyzer.report_error(&ReturnVoid, &return_expr.span);
								}
								if *actual_return_type.kind != parent_fn_return_type {
									analyzer.report_error(&ReturnBad, &return_expr.span);
								}
							},
							None => ()
						}
					},
					None => {
						if parent_fn_return_type != Prim(PrimType::Void) {
							analyzer.report_error(&ReturnMissing, &self.span);
						}
					}
				}
			},
			Exit => (),
			Call(ref mut fn_call) => {
				fn_call.eval_type(analyzer);
			},
		}
	}
}

impl TypeAnalysisNode for Exp {
	fn analyze_type(&mut self, analyzer: &mut TypeAnalyzer) {
		use ExpKind::*;
		let kind = *self.kind.clone();
		let expr_type = match kind {
			UnaryExp(mut x) => x.eval_type(analyzer),
			BinaryExp(mut x) => x.eval_type(analyzer),
			CallExp(mut x) => x.eval_type(analyzer), //return type
			Loc(mut x) => x.eval_type(analyzer),
			_ => Err(())
		};
		
		match expr_type {
			Ok(x) => self.expr_type = Some(Box::new(x)),
			Err(()) => ()
		}
	}
}

impl EvaluateExpType for UnaryExp {
    fn eval_type(&mut self, analyzer: &mut TypeAnalyzer) -> Result<Type, ()> {
		use TypeKind::*;
		use UnaryExpKind::*;

		self.exp.analyze_type(analyzer);

		let expr_type = match self.exp.expr_type.clone() {
			Some(x) => *x,
			None => return Err(())
		};

		// error check
		let kind = *self.kind.clone();
		match kind {
			Neg => {
				match &*expr_type.kind {
					&Prim(PrimType::Int) => {
						self.expr_type = self.exp.expr_type.clone();
						Ok(expr_type)
					},
					_ => {
						analyzer.report_error(&WrongOpMath, &self.exp.span);
						Err(())
					}
				}
			},
			Not => {
				match &*expr_type.kind {
					&Prim(PrimType::Bool) => {
						self.expr_type = self.exp.expr_type.clone(); 
						Ok(expr_type)
					},
					_ => {
						analyzer.report_error(&WrongOpCmp, &self.exp.span);
						Err(())
					}
				}
			},
		}
	}
}

impl EvaluateExpType for BinaryExp {
    fn eval_type(&mut self, analyzer: &mut TypeAnalyzer) -> Result<Type, ()> {
		use BinaryExpKind::*;

		self.lhs.analyze_type(analyzer);
		self.rhs.analyze_type(analyzer);

		let lhs_type = match &self.lhs.expr_type {
			Some(x) => *x.clone(),
			None => return Err(())
		};
		let rhs_type = match &self.rhs.expr_type {
			Some(x) => *x.clone(),
			None => return Err(())
		};

		let lhs_type_kind = *lhs_type.clone().kind;
		let rhs_type_kind = *rhs_type.clone().kind;

		// error check
		match *self.kind {
			Plus | Minus | Times | Divide => {
				self.eval_type_helper(lhs_type, 
					lhs_type_kind, rhs_type_kind, 
					PrimType::Int, WrongOpMath, analyzer
				)
			},
			And | Or => {
				self.eval_type_helper(lhs_type, 
					lhs_type_kind, rhs_type_kind, 
					PrimType::Bool, WrongOpCmp, analyzer
				)
			},
			Less | Greater | LessEq | GreaterEq  => {
				self.eval_type_helper(lhs_type, 
					lhs_type_kind, rhs_type_kind, 
					PrimType::Int, WrongOpLogic, analyzer
				)
			},
			//TODO: this
			Equals | NotEquals  => {
				// if lhs_type_kind != lhs_type_kind {
				// }
				Err(())
			},
		}
	}
}

impl BinaryExp {
	fn eval_type_helper(&mut self, lhs_type: Type, 
			lhs_type_kind: TypeKind, rhs_type_kind: TypeKind,
			correct_type: PrimType, err: ErrorType, analyzer: &mut TypeAnalyzer
	) -> Result<Type, ()> {
		use TypeKind::*;
		let mut error = false;
		if lhs_type_kind != Prim(correct_type) {
			analyzer.report_error(&err, &self.lhs.span);
			error = true;
		}
		if rhs_type_kind != Prim(correct_type) {
			analyzer.report_error(&err, &self.rhs.span);
			error = true;
		}
		if !error {
			self.expr_type = self.lhs.expr_type.clone();
			Ok(lhs_type)
		} else {
			Err(())
		}
	}
}

impl EvaluateExpType for CallExp {
    fn eval_type(&mut self, analyzer: &mut TypeAnalyzer) -> Result<Type, ()> {
		//TODO: this
		let binding = analyzer.clone();
		let func = binding.get_fn(&self.name.to_string());
		
		if func.is_err() { // couldn't find in fn list, called a non func
			analyzer.report_error(&CallNonFn, &self.name.span);
			return Err(())
		}

		let func = func?;
		let mut error = false;
		let arg_num = if self.args.len() != func.arg_types.len() {
			analyzer.report_error(&FnWrongArgNum, &self.span);
			error = true;
			min(self.args.len(), func.arg_types.len())
		} else {
			self.args.len()
		};
		for i in 0..arg_num {
			self.args[i].analyze_type(analyzer);
			match self.args[i].expr_type.clone() {
				Some(actual) => {
					if *actual != func.arg_types[i] {
						analyzer.report_error(&FnWrongArgType, &self.args[i].span);
						error = true;
					}
				},
				None => error = true,
			}
		}

		if !error {
			self.fn_type = Some(func.return_type.clone());
			Ok(func.return_type.clone())
		} else {
			Err(())
		}
	}
}

impl EvaluateExpType for Loc {
    fn eval_type(&mut self, analyzer: &mut TypeAnalyzer) -> Result<Type, ()> {
		//TODO: this
		Err(())
	}
}