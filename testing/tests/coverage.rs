extern crate askama_compile;
extern crate runtime_macros;
extern crate syn;

use std::env;
use std::fs;

#[test]
fn proc_macro_coverage() {
    let mut test_dir = env::current_dir().unwrap();
    test_dir.push("tests");
    for entry in fs::read_dir(&test_dir).unwrap() {
        let f = fs::File::open(entry.unwrap().path()).unwrap();
        runtime_macros::emulate_macro_expansion(f, "proc_macro_derive", |tokens| {
            let ast: syn::DeriveInput = syn::parse2(tokens).unwrap();
            println!("IDENT: {}", ast.ident);
            askama_compile::build_template(&ast).parse().unwrap()
        });
    }
}
