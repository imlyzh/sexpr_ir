use crate::gast::constant::Constant;



pub struct Macro {

}

pub enum Pattern {
    Ignore,
    Const(Constant)
}
