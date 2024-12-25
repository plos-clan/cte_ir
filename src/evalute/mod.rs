use crate::ir::Program;
use std::sync::RwLock;

mod program;

pub struct Evaluter<'a> {
    program: &'a mut Program,
}

impl Evaluter<'_> {
    pub fn evalute(&mut self) {
        self.program.evalute(RwLock::new(EvalutionContext::new()));
    }
}

struct EvalutionContext {
    
}

impl EvalutionContext {
    pub fn new() -> Self {
        Self {
            
        }
    }
}

trait Evalute {
    fn evalute(&mut self, context: RwLock<EvalutionContext>);
}
