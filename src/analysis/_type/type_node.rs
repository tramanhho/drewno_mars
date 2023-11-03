use super::*;
use crate::parser::ast::*;
use super::TypeAnalyzer;

pub trait TypeAnalysisNode {
    fn analyze_type(&mut self, analyzer: &mut TypeAnalyzer);
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
            VarDecl(_) => (),
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
	}
}