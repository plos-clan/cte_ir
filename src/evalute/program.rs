use super::*;
use crate::ir::*;

impl Program {
    pub fn new_evaluter(&mut self) -> Evaluter<'_> {
        Evaluter { program: self }
    }
}

impl Evalute for Program {
    fn evalute(&mut self, _context: RwLock<EvalutionContext>) {
        
    }
}

