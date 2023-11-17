use super::*;
use super::TypeAnalyzer;
use super::ErrorType::*;
use std::cmp::min;
pub trait TypeAnalysisNode {
    fn analyze_type(&mut self, analyzer: &mut TypeAnalyzer);
}

pub trait TypeAnalysisStmtNode {
    fn analyze_type(&mut self, analyzer: &mut TypeAnalyzer, return_type: &TypeKind);
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

impl TypeAnalysisNode for ClassDecl {
	fn analyze_type(&mut self, analyzer: &mut TypeAnalyzer) {	
		analyzer.add_class(self);
	}
}

impl TypeAnalysisNode for FnDecl {
	fn analyze_type(&mut self, analyzer: &mut TypeAnalyzer) {
		analyzer.add_fn(self);

        let return_type = *self.ret.kind.clone();
        analyzer.scope += 1;

        use self::FormalDecl::*;
        for arg in self.args.iter() {
			let (id, var_type) = match *arg.clone() {
                VarDecl(x) => (x.id, x.var_type),
                FormalDecl { id, formal_type } => (id, formal_type)
            };
            analyzer.add_var(id.to_string(), *var_type.clone());
		}

		for stmt in self.body.iter_mut() {
			stmt.analyze_type(analyzer, &return_type);
		}
        analyzer.remove_scope();
        analyzer.scope -= 1;
	}
}

impl TypeAnalysisNode for VarDecl {
	fn analyze_type(&mut self, analyzer: &mut TypeAnalyzer) {
        use TypeKind::*;

		match self.init_val {
			Some(ref mut init) => { 
                init.analyze_type(analyzer); 
                if self.var_type.perfect == true {
					analyzer.report_error(&NonLval, &self.id.span);
				}

                match *self.var_type.kind.clone() {
                    Class(_) => analyzer.report_error(&BadAssignOne, &self.id.span),
                    _ => ()
                }

                match &init.expr_type {
                    Some(expr_type) => {
                        if *expr_type != self.var_type {
                            analyzer.report_error(&BadAssignOne, &init.span);
                        }
                    },
                    None => ()
                };
            },
			None => (),
		};

        match *self.var_type.kind.clone() {
            Class(class) => analyzer.add_class_inst(self.id.to_string(), class.name),
            _ => analyzer.add_var(self.id.to_string(), *self.var_type.clone()),
        };
        // println!("{}", analyzer);
	}
}

impl TypeAnalysisStmtNode for Stmt {
	fn analyze_type(&mut self, analyzer: &mut TypeAnalyzer, return_type: &TypeKind) {
		use Stmt::*;

		match self {
			Block(x) => {
                analyzer.scope += 1;
                x.analyze_type(analyzer, return_type);
                analyzer.remove_scope();
                analyzer.scope -= 1;
            },
			Line(x) => x.analyze_type(analyzer, return_type),
			VarDecl(x) => x.analyze_type(analyzer),
		}
	}
}

impl TypeAnalysisStmtNode for BlockStmt {
	fn analyze_type(&mut self, analyzer: &mut TypeAnalyzer, return_type: &TypeKind) {
		use BlockStmt::*;

		match self {
			While {ref mut cond, ref mut body} | 
            If{ref mut cond, ref mut body} => {
                cond.analyze_type(analyzer);
                // println!("{}", cond);
                match is_condition_bool(cond) {
                    Ok(cond_is_bool) => {
                        if !cond_is_bool {
                            analyzer.report_error(&CondNonBool, &cond.span);
                        } 
                    }
                    Err(()) => ()
                }

                for stmt in body.iter_mut() {
                    stmt.analyze_type(analyzer, return_type);
                }
            },
			IfElse{ref mut cond, ref mut true_branch, ref mut false_branch, } => {
                cond.analyze_type(analyzer);
                match is_condition_bool(cond) {
                    Ok(cond_is_bool) => {
                        if !cond_is_bool {
                            analyzer.report_error(&CondNonBool, &cond.span);
                        } 
                    }
                    Err(()) => ()
                }

                for stmt in true_branch.iter_mut() {
                    stmt.analyze_type(analyzer, return_type);
                }
                for stmt in false_branch.iter_mut() {
                    stmt.analyze_type(analyzer, return_type);
                }
            },
		}
	}
}

fn is_condition_bool(cond: &mut Box<Exp>) -> Result<bool, ()> {
    use TypeKind::*;
    match cond.expr_type.clone() {
        Some(expr_type) => {
            match *expr_type.kind {
                Prim(PrimType::Bool) => Ok(true),
                _ => Ok(false)
            }
        },
        None => Err(())
    }
}

impl TypeAnalysisStmtNode for LineStmt {
	fn analyze_type(&mut self, analyzer: &mut TypeAnalyzer, parent_fn_return_type: &TypeKind) {
		let mut variation = *self.kind.clone();

		use LineStmtKind::*;
		use TypeKind::*;
		use ExpKind::*;
		match variation {
			Assign { ref mut dest, ref mut src } => {
                
				let dest_type = dest.eval_type(analyzer);
				src.analyze_type(analyzer);
				let src_type: &Option<Box<Type>> = &src.expr_type;

                let mut invalid_operators = false;
                if dest.is_fn_or_class(analyzer) {
                    analyzer.report_error(&BadAssignOne, &dest.span);
                    invalid_operators = true;
                }

                if src.is_fn_or_class(analyzer){
                    analyzer.report_error(&BadAssignOne, &src.span);
                    invalid_operators = true;
                }

                if dest_type.is_err() || src_type.is_none() {
					return;
				}

                let dest_type = dest_type.unwrap();
                let src_type = *src_type.clone().unwrap();

                if dest_type.perfect == true {
                    analyzer.report_error(&NonLval, &dest.span);
                    invalid_operators = true;
                }

                if src_type.perfect == true {
                    analyzer.report_error(&NonLval, &src.span);
                    invalid_operators = true;
                }

                if invalid_operators {
                    return;
                }

                if dest_type != src_type {
                    analyzer.report_error(&BadAssignTwo, &self.span);
                }
				
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
                        // println!("{}", return_expr);
						match return_expr.expr_type.clone() {
							Some(actual_return_type) => {
                                // println!("{}, {}", &actual_return_type.kind, &parent_fn_return_type);
								if *actual_return_type.kind != Prim(PrimType::Void) && 
                                    parent_fn_return_type == &Prim(PrimType::Void) {
									analyzer.report_error(&ReturnVoid, &return_expr.span);
								} else {
                                    if &*actual_return_type.kind != parent_fn_return_type {
                                        analyzer.report_error(&ReturnBad, &return_expr.span);
                                    }
                                }
							},
							None => ()
						}
					},
					None => {
						if parent_fn_return_type != &Prim(PrimType::Void) {
							analyzer.report_error(&ReturnMissing, &self.span);
						}
					}
				}
			},
			Exit => (),
			Call(ref mut fn_call) => {
				let _ = fn_call.eval_type(analyzer);
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
                if &*expr_type.kind != &Prim(PrimType::Int) {
                    analyzer.report_error(&WrongOpMath, &self.exp.span);
                    return Err(());
                } else {
                    self.expr_type = self.exp.expr_type.clone();
                    return Ok(expr_type);
                }
			},
			Not => {
                if &*expr_type.kind != &Prim(PrimType::Bool) {
                    analyzer.report_error(&WrongOpLogic, &self.exp.span);
                    return Err(());
                } else {
                    self.expr_type = self.exp.expr_type.clone();
                    return Ok(expr_type);
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
			Some(x) => Some(*x.clone()),
			None => None
		};
		let rhs_type = match &self.rhs.expr_type {
			Some(x) => Some(*x.clone()),
			None => None
		};

		let lhs_type_kind = match lhs_type.clone() {
			Some(x) => Some(*x.clone().kind),
			None => None
		};

		let rhs_type_kind = match rhs_type.clone() {
			Some(x) => Some(*x.clone().kind),
			None => None
		};

		// error check
		match *self.kind {
			Plus | Minus | Times | Divide => {
				return self.eval_type_helper(lhs_type, 
					lhs_type_kind, rhs_type_kind, 
					PrimType::Int, WrongOpMath, analyzer
				)
			},
			And | Or => {
				return self.eval_type_helper(lhs_type, 
					lhs_type_kind, rhs_type_kind, 
					PrimType::Bool, WrongOpLogic, analyzer
				)
			},
			Less | Greater | LessEq | GreaterEq  => {
				return self.eval_type_helper(lhs_type, 
					lhs_type_kind, rhs_type_kind, 
					PrimType::Int, WrongOpCmp, analyzer
				)
			},
			Equals | NotEquals  => {
                let mut invalid_operands = false;
                
                if self.lhs.is_fn_or_class(analyzer) {
                    analyzer.report_error(&BadEqualityOne, &self.lhs.span);
                    invalid_operands = true;
                }
                if self.rhs.is_fn_or_class(analyzer) {
                    analyzer.report_error(&BadEqualityOne, &self.rhs.span);
                    invalid_operands = true;
                }

                use TypeKind::*;

                if lhs_type_kind.is_some() && lhs_type_kind.clone().unwrap() == Prim(PrimType::Void) {
                    analyzer.report_error(&BadEqualityOne, &self.lhs.span);
                    invalid_operands = true;
                }

                if rhs_type_kind.is_some() && rhs_type_kind.clone().unwrap() == Prim(PrimType::Void) {
                    analyzer.report_error(&BadEqualityOne, &self.rhs.span);
                    invalid_operands = true;
                }
                
                if invalid_operands {
                    return Err(());
                }

                
                if lhs_type_kind != rhs_type_kind {
                    analyzer.report_error(&BadEqualityTwo, &self.span);
                    return Err(());
                }
				
                self.expr_type = self.lhs.expr_type.clone();
                if lhs_type.is_some() {
                    return Ok(lhs_type.unwrap());
                } else {
                    return Err(())
                }
			},
		}
	}
}

impl Exp {
    fn is_fn_or_class(&self, analyzer: &mut TypeAnalyzer) -> bool {
        use ExpKind::*;
        match *self.kind.clone() {
            Loc(location) => {
                analyzer.has_class(&location.to_string()) ||
                analyzer.has_fn(&location.to_string()) 
            }
            _ => false
        }
    }
}

impl Loc {
    fn is_fn_or_class(&self, analyzer: &mut TypeAnalyzer) -> bool {
        analyzer.has_class(&self.to_string()) ||
        analyzer.has_fn(&self.to_string()) 
    }
}

impl BinaryExp {
	fn eval_type_helper(&mut self, lhs_type: Option<Type>, 
			lhs_type_kind: Option<TypeKind>, rhs_type_kind: Option<TypeKind>,
			correct_type: PrimType, err: ErrorType, analyzer: &mut TypeAnalyzer
	) -> Result<Type, ()> {
        if lhs_type.is_none() || lhs_type_kind.is_none() || rhs_type_kind.is_none() {
            return Err(())
        }

        let lhs_type = lhs_type.unwrap();
        let lhs_type_kind = lhs_type_kind.unwrap();
        let rhs_type_kind = rhs_type_kind.unwrap();

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
		let my_type = analyzer.get_var_type(self.to_string());
        match my_type.clone() {
            Ok(x) => self.loc_type = Some(x),
            Err(()) => ()
        };
        my_type
	}
}