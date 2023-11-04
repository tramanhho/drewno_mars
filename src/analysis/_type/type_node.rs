use super::*;
use super::TypeAnalyzer;
use super::ErrorType::*;

pub trait TypeAnalysisNode {
    fn analyze_type(&mut self, analyzer: &mut TypeAnalyzer);
}

impl TypeAnalysisNode for Program {
	fn analyze_type(&mut self, analyzer: &mut TypeAnalyzer) {
		println!("in program!");
		for gbl in self.globals.iter_mut() {
			gbl.analyze_type(analyzer);
		}
	}
}

impl TypeAnalysisNode for Decl {
	fn analyze_type(&mut self, analyzer: &mut TypeAnalyzer) {
		use Decl::*;
		println!("in decl!");
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

		for stmt in self.body.iter_mut() {
			stmt.analyze_type(analyzer);
		}
	}
}

impl TypeAnalysisNode for VarDecl {
	fn analyze_type(&mut self, analyzer: &mut TypeAnalyzer) {
		println!("in vardecl {}!", &self);
		let init = match self.init_val {
			Some(ref mut x) => { x.analyze_type(analyzer); x },
			None => return,
		};
		match &init.expr_type {
			Some(expr_type) => {
				println!("{}, {}", expr_type, &self.var_type);
				if *expr_type != self.var_type {
					analyzer.report_error(BadAssign, init.span);
				}
			},
			None => ()
		}
	}
}

impl TypeAnalysisNode for Stmt {
	fn analyze_type(&mut self, analyzer: &mut TypeAnalyzer) {
		use Stmt::*;

		match self {
			Block(x) => x.analyze_type(analyzer),
			Line(x) => x.analyze_type(analyzer),
			VarDecl(x) => x.analyze_type(analyzer),
		}
	}
}

impl TypeAnalysisNode for BlockStmt {
	fn analyze_type(&mut self, analyzer: &mut TypeAnalyzer) {
		
	}
}

impl TypeAnalysisNode for LineStmt {
	fn analyze_type(&mut self, analyzer: &mut TypeAnalyzer) {
		
	}
}

impl TypeAnalysisNode for Exp {
	fn analyze_type(&mut self, analyzer: &mut TypeAnalyzer) {
		
	}
}