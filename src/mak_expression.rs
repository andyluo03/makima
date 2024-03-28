use std::rc::Rc;

pub struct Expression {
    contents_: String,
    arguments_: Vec<Rc<Expression>>,
    is_atom_: bool,
}
