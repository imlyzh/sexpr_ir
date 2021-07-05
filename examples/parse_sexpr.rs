use sexpr_ir::syntax::sexpr::{repl_parse, parse};

macro_rules! construct_demo {
    ($e:expr) => {
        println!("input: {}", $e);
        let r = repl_parse($e).unwrap();
        println!("repl_parse: {}", r);
        let r = parse($e).unwrap();
        println!("unit_parse: {:?}", r);
    };
}

fn main() {
    construct_demo!("foo");
    construct_demo!("'bar");
    construct_demo!("`bar");
    construct_demo!("()");
    construct_demo!("(if blabla then blabla else blabla)");
    construct_demo!("(assert henghengheng 114514)");
    construct_demo!("(doc \"document\")");
    construct_demo!("(doc \"
        # this is a test\\n
        ## abaaba\\n
        \\tyukikaze sama nanoda\n\")");
    construct_demo!("(config [
        (name . \"hoshino tented\")
        (age . 512)
        (gender . female)
        {abaaba}
    ])");
    construct_demo!("
    (let [a 1]
        [b -1]
        [c 1.1]
        [d -2.1]
        '(a b c . e)
        `(+ a b c d))");
}
