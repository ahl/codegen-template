use codegen::codegen_attr;
use codegen::codegen_macro;
use codegen::Codegen;

/// Show the three different uses of our simple codegen macros
fn main() {
    macro1();
    macro2();
    macro3();
}

fn macro1() {
    codegen_macro!();
    print_hi();
}

fn macro2() {
    #[allow(dead_code)]
    #[derive(Codegen)]
    struct Foo(String);
    print_hi();
}

fn macro3() {
    #[allow(dead_code)]
    #[codegen_attr]
    struct Foo(String);
    print_hi();
}
