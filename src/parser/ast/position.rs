use super::*;
use std::fmt::{Display, Formatter, Error};

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
	start_row: usize,
	start_col: usize,
	end_row: usize,
	end_col: usize
}

impl Position {
	pub fn new(start: usize, end: usize) -> Position {
		Position {
			start_row: 1,
			start_col: start,
			end_row: 1,
			end_col: end
		}
	}
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "[{}, {}]-[{}, {}]", self.start_row, self.start_col, self.end_row, self.end_col)
    }
}

impl Position {
	fn correct(&mut self, line_bytes: &Vec<usize>) {
	self.start_col += 1;
	self.end_col += 1;

	let mut line_bytes = line_bytes.into_iter();
	loop {
		let line = match line_bytes.next() {
			Some(x) => x + 2,
			None => break,
		};

		if self.start_col >= line {
			self.start_col -= line;
			self.start_row += 1;
		} else {
			()
		}

		if self.end_col >= line {
			self.end_col -= line;
			self.end_row += 1;
		} else {
			break;
		}
	}
}
}

pub fn line_bytes(input: String) -> Vec<usize> {
	let lines = input.lines();
	lines.map(|line| line.len()).collect::<Vec<usize>>()
}

pub trait PositionAPI {
	fn correct_position_rec(&mut self, line_bytes: &Vec<usize>);
	// fn position(&self) -> Range<usize>;
}

impl PositionAPI for Program {
	fn correct_position_rec(&mut self, line_bytes: &Vec<usize>) {
		for gbl in self.globals.iter_mut() {
			gbl.correct_position_rec(line_bytes);
		}
	}
}

impl PositionAPI for Decl {
	fn correct_position_rec(&mut self, line_bytes: &Vec<usize>) {
		use Decl::*;

		match self {
            VarDecl(x) => x.correct_position_rec(line_bytes),
            ClassDecl(x) => x.correct_position_rec(line_bytes),
            FnDecl(x) => x.correct_position_rec(line_bytes),
        };
	}
}

impl PositionAPI for VarDecl {
	fn correct_position_rec(&mut self, line_bytes: &Vec<usize>) {
		self.id.correct_position_rec(line_bytes);
	}
}

impl PositionAPI for ClassDecl {
	fn correct_position_rec(&mut self, line_bytes: &Vec<usize>) {
		self.id.correct_position_rec(line_bytes);
		for field in self.member_f.iter_mut() {
			field.correct_position_rec(line_bytes);
		}
	}
}

impl PositionAPI for FnDecl {
	fn correct_position_rec(&mut self, line_bytes: &Vec<usize>) {
		self.id.correct_position_rec(line_bytes);
		for field in self.args.iter_mut() {
			field.correct_position_rec(line_bytes);
		}
		for field in self.body.iter_mut() {
			field.correct_position_rec(line_bytes);
		}
	}
}

impl PositionAPI for FormalDecl {
	fn correct_position_rec(&mut self, line_bytes: &Vec<usize>) {
		use super::FormalDecl::*;

		match self {
            VarDecl(x) => x.correct_position_rec(line_bytes),
            FormalDecl{id, formal_type: _} => id.correct_position_rec(line_bytes),
        };
	}
}

impl PositionAPI for Stmt {
	fn correct_position_rec(&mut self, line_bytes: &Vec<usize>) {
		use Stmt::*;

		match self {
            Block(x) => x.correct_position_rec(line_bytes),
            Line(x) => x.correct_position_rec(line_bytes),
			VarDecl(x) => x.correct_position_rec(line_bytes),
        };
	}
}

impl PositionAPI for BlockStmt {
	fn correct_position_rec(&mut self, line_bytes: &Vec<usize>) {
		use BlockStmt::*;

		match self {
            While{cond, body} => {
				cond.correct_position_rec(line_bytes);
				for stmt in body.iter_mut() {
					stmt.correct_position_rec(line_bytes);
				}
			},
            If{cond, body} => {
				cond.correct_position_rec(line_bytes);
				for stmt in body.iter_mut() {
					stmt.correct_position_rec(line_bytes);
				}
			},
            IfElse{cond, true_branch, false_branch} => {
				cond.correct_position_rec(line_bytes);
				for stmt in true_branch.iter_mut() {
					stmt.correct_position_rec(line_bytes);
				}
				for stmt in false_branch.iter_mut() {
					stmt.correct_position_rec(line_bytes);
				}
			},
        };
	}
}

impl PositionAPI for LineStmt {
	fn correct_position_rec(&mut self, line_bytes: &Vec<usize>) {
		use LineStmt::*;

		match self {
            Assign{dest, src} => {
				dest.correct_position_rec(line_bytes);
				src.correct_position_rec(line_bytes);
			},
            PostDec{loc} => loc.correct_position_rec(line_bytes),
            PostInc{loc} => loc.correct_position_rec(line_bytes),
            Give{output} => output.correct_position_rec(line_bytes),
            Take{recipient} => recipient.correct_position_rec(line_bytes),
            Return{result} => {match result {
                Some(x) => x.correct_position_rec(line_bytes),
                None => (),}},
            Exit => (),
            Call(exp) => exp.correct_position_rec(line_bytes),
        }
	}
}

impl PositionAPI for Exp {
    fn correct_position_rec(&mut self, line_bytes: &Vec<usize>) {
        use Exp::*;
        match self {
            True => (),
            False => (),
            Magic => (),
            UnaryExp(exp) => exp.correct_position_rec(line_bytes),
            BinaryExp(exp) => exp.correct_position_rec(line_bytes),
            CallExp(exp) => exp.correct_position_rec(line_bytes),
            // IntLit(lit) => lit.correct_position_rec(line_bytes),
            // StrLit(lit) => lit.correct_position_rec(line_bytes),
            Loc(loc) => loc.correct_position_rec(line_bytes),
			_ => (), // handle lits later
        }
    }
}

impl PositionAPI for UnaryExp {
    fn correct_position_rec(&mut self, line_bytes: &Vec<usize>) {
        use UnaryExp::*;
        match self {
            Neg{exp} => exp.correct_position_rec(line_bytes),
            Not{exp} => exp.correct_position_rec(line_bytes),
        }
    }
}

impl PositionAPI for BinaryExp {
    fn correct_position_rec(&mut self, line_bytes: &Vec<usize>) {
        use BinaryExp::*;
        match self {
            And{lhs, rhs} => {
				lhs.correct_position_rec(line_bytes);
				rhs.correct_position_rec(line_bytes);
			},
            Or{lhs, rhs} => {
				lhs.correct_position_rec(line_bytes);
				rhs.correct_position_rec(line_bytes);
			},
            Equals{lhs, rhs} => {
				lhs.correct_position_rec(line_bytes);
				rhs.correct_position_rec(line_bytes);
			},
            NotEquals{lhs, rhs} => {
				lhs.correct_position_rec(line_bytes);
				rhs.correct_position_rec(line_bytes);
			},
            Greater{lhs, rhs} => {
				lhs.correct_position_rec(line_bytes);
				rhs.correct_position_rec(line_bytes);
			},
            Less{lhs, rhs} => {
				lhs.correct_position_rec(line_bytes);
				rhs.correct_position_rec(line_bytes);
			},
            GreaterEq{lhs, rhs} => {
				lhs.correct_position_rec(line_bytes);
				rhs.correct_position_rec(line_bytes);
			},
            LessEq{lhs, rhs} => {
				lhs.correct_position_rec(line_bytes);
				rhs.correct_position_rec(line_bytes);
			},
            Plus{lhs, rhs} => {
				lhs.correct_position_rec(line_bytes);
				rhs.correct_position_rec(line_bytes);
			},
            Minus{lhs, rhs} => {
				lhs.correct_position_rec(line_bytes);
				rhs.correct_position_rec(line_bytes);
			},
            Times{lhs, rhs} => {
				lhs.correct_position_rec(line_bytes);
				rhs.correct_position_rec(line_bytes);
			},
            Divide{lhs, rhs} => {
				lhs.correct_position_rec(line_bytes);
				rhs.correct_position_rec(line_bytes);
			},
        }
    }
}

impl PositionAPI for CallExp {
    fn correct_position_rec(&mut self, line_bytes: &Vec<usize>) {
		self.name.correct_position_rec(line_bytes);
		for arg in self.args.iter_mut() {
			arg.correct_position_rec(line_bytes);
		}
    }
}

impl PositionAPI for Loc {
    fn correct_position_rec(&mut self, line_bytes: &Vec<usize>) {
        use self::Loc::*;
        match self {
            Id(x) => x.correct_position_rec(line_bytes),
            Loc { base_class , field_name} => {
				base_class.correct_position_rec(line_bytes);
				field_name.correct_position_rec(line_bytes);
			},
        }
    }
}

impl PositionAPI for Id {
	fn correct_position_rec(&mut self, line_bytes: &Vec<usize>) {
		self.position.correct(line_bytes);
	}
}