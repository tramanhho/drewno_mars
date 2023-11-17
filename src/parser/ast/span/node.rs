use super::super::*;

pub trait SpanNode {
	fn correct_span_rec(&mut self, line_bytes: &Vec<usize>);
}

impl SpanNode for Program {
	fn correct_span_rec(&mut self, line_bytes: &Vec<usize>) {
		for gbl in self.globals.iter_mut() {
			gbl.correct_span_rec(line_bytes);
		}
	}
}

impl SpanNode for Decl {
	fn correct_span_rec(&mut self, line_bytes: &Vec<usize>) {
		use Decl::*;

		match self {
            VarDecl(x) => x.correct_span_rec(line_bytes),
            ClassDecl(x) => x.correct_span_rec(line_bytes),
            FnDecl(x) => x.correct_span_rec(line_bytes),
        };
	}
}

impl SpanNode for VarDecl {
	fn correct_span_rec(&mut self, line_bytes: &Vec<usize>) {
		self.id.correct_span_rec(line_bytes);
		match self.init_val {
			Some(ref mut init_val) => init_val.correct_span_rec(line_bytes),
			None => ()
		}
	}
}

impl SpanNode for ClassDecl {
	fn correct_span_rec(&mut self, line_bytes: &Vec<usize>) {
		self.id.correct_span_rec(line_bytes);
		for field in self.member_f.iter_mut() {
			field.correct_span_rec(line_bytes);
		}
	}
}

impl SpanNode for FnDecl {
	fn correct_span_rec(&mut self, line_bytes: &Vec<usize>) {
		self.id.correct_span_rec(line_bytes);
		for field in self.args.iter_mut() {
			field.correct_span_rec(line_bytes);
		}
		for field in self.body.iter_mut() {
			field.correct_span_rec(line_bytes);
		}
	}
}

impl SpanNode for FormalDecl {
	fn correct_span_rec(&mut self, line_bytes: &Vec<usize>) {
		use crate::parser::ast::FormalDecl::*;

		match self {
            VarDecl(x) => x.correct_span_rec(line_bytes),
            FormalDecl{id, formal_type: _} => id.correct_span_rec(line_bytes),
        };
	}
}

impl SpanNode for Stmt {
	fn correct_span_rec(&mut self, line_bytes: &Vec<usize>) {
		use Stmt::*;

		match self {
            Block(x) => x.correct_span_rec(line_bytes),
            Line(x) => x.correct_span_rec(line_bytes),
			VarDecl(x) => x.correct_span_rec(line_bytes),
        };
	}
}

impl SpanNode for BlockStmt {
	fn correct_span_rec(&mut self, line_bytes: &Vec<usize>) {
		use BlockStmt::*;

		match self {
            While{cond, body} => {
				cond.correct_span_rec(line_bytes);
				for stmt in body.iter_mut() {
					stmt.correct_span_rec(line_bytes);
				}
			},
            If{cond, body} => {
				cond.correct_span_rec(line_bytes);
				for stmt in body.iter_mut() {
					stmt.correct_span_rec(line_bytes);
				}
			},
            IfElse{cond, true_branch, false_branch} => {
				cond.correct_span_rec(line_bytes);
				for stmt in true_branch.iter_mut() {
					stmt.correct_span_rec(line_bytes);
				}
				for stmt in false_branch.iter_mut() {
					stmt.correct_span_rec(line_bytes);
				}
			},
        };
	}
}

impl SpanNode for LineStmt {
	fn correct_span_rec(&mut self, line_bytes: &Vec<usize>) {
		self.span.correct(line_bytes);
		self.kind.correct_span_rec(line_bytes);
	} 
}

impl SpanNode for LineStmtKind {
	fn correct_span_rec(&mut  self, line_bytes: &Vec<usize>) {
		use LineStmtKind::*;

		match self {
            Assign{dest, src} => {
				dest.correct_span_rec(line_bytes);
				src.correct_span_rec(line_bytes);
			},
            PostDec{loc} => loc.correct_span_rec(line_bytes),
            PostInc{loc} => loc.correct_span_rec(line_bytes),
            Give{output} => output.correct_span_rec(line_bytes),
            Take{recipient} => recipient.correct_span_rec(line_bytes),
            Return{result} => {match result {
                Some(x) => x.correct_span_rec(line_bytes),
                None => (),}},
            Exit => (),
            Call(exp) => exp.correct_span_rec(line_bytes),
        }
	} 
}

impl SpanNode for Exp {
    fn correct_span_rec(&mut self, line_bytes: &Vec<usize>) {
		self.span.correct(line_bytes);
        self.kind.correct_span_rec(line_bytes);
    }
}

impl SpanNode for ExpKind {
    fn correct_span_rec(&mut self, line_bytes: &Vec<usize>) {
		use ExpKind::*;
        match self {
            True => (),
            False => (),
            Magic => (),
            UnaryExp(exp) => exp.correct_span_rec(line_bytes),
            BinaryExp(exp) => exp.correct_span_rec(line_bytes),
            CallExp(exp) => exp.correct_span_rec(line_bytes),
            // IntLit(lit) => lit.correct_span_rec(line_bytes),
            // StrLit(lit) => lit.correct_span_rec(line_bytes),
            Loc(loc) => loc.correct_span_rec(line_bytes),
			_ => (), // handle lits later
        }
    }
}

impl SpanNode for UnaryExp {
    fn correct_span_rec(&mut self, line_bytes: &Vec<usize>) {
		self.span.correct(line_bytes);
        self.exp.correct_span_rec(line_bytes);
    }
}

impl SpanNode for BinaryExp {
    fn correct_span_rec(&mut self, line_bytes: &Vec<usize>) {
		self.span.correct(line_bytes);
        self.lhs.correct_span_rec(line_bytes);
		self.rhs.correct_span_rec(line_bytes);
    }
}

impl SpanNode for CallExp {
    fn correct_span_rec(&mut self, line_bytes: &Vec<usize>) {
		self.span.correct(line_bytes);
		self.name.correct_span_rec(line_bytes);
		for arg in self.args.iter_mut() {
			arg.correct_span_rec(line_bytes);
		}
    }
}

impl SpanNode for Loc {
    fn correct_span_rec(&mut self, line_bytes: &Vec<usize>) {
		self.span.correct(line_bytes);
        self.kind.correct_span_rec(line_bytes)
    }
}

impl SpanNode for LocKind {
    fn correct_span_rec(&mut self, line_bytes: &Vec<usize>) {
        use self::LocKind::*;
        match self {
            Id(x) => x.correct_span_rec(line_bytes),
            Loc { base_class , field_name} => {
				base_class.correct_span_rec(line_bytes);
				field_name.correct_span_rec(line_bytes);
			},
        }
    }
}

impl SpanNode for Id {
	fn correct_span_rec(&mut self, line_bytes: &Vec<usize>) {
		self.span.correct(line_bytes);
	}
}