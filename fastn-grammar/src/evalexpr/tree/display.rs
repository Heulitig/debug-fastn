use fastn_grammar::evalexpr::ExprNode;
use std::fmt::{Display, Error, Formatter};

impl Display for ExprNode {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self.operator.fmt(f)?;
        for child in self.children() {
            write!(f, " {}", child)?;
        }
        Ok(())
    }
}
