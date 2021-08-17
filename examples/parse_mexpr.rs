use sexpr_ir::syntax::mexpr::{one_unit_parse};


macro_rules! construct_demo {
    ($e:expr) => {
        println!("input: {}", $e);
        let r = one_unit_parse($e, "<examples>").unwrap();
        println!("parse: {}", r);
    };
}

fn main() {
    construct_demo!("foo");
    construct_demo!("nil");
    construct_demo!("true");
    construct_demo!("false");
    construct_demo!("()");
    construct_demo!("[]");
    construct_demo!("{}");
    construct_demo!("if[cond; then; else]");
    construct_demo!("case[cond; then; else]");
    construct_demo!("assert(henghengheng, 114514)");
    construct_demo!("doc[\"document\"]");
    construct_demo!("{1, 2 ,3}");
    construct_demo!("(1, 2, 3)");
}
