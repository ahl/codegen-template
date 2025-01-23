use proc_macro2::TokenStream;
use quote::quote;

/// This is our trivial code generation; one imagines we might want to do
/// something fancier, but this is sufficient to demonstrate how we might use
/// this collection of packages.
pub fn do_codegen() -> TokenStream {
    quote! {
        fn print_hi() {
            println!("I guess this is working!");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::do_codegen;

    // You can write simple tests that validate output, but those often require
    // weird formatting that can be challenging to get right.
    #[test]
    fn simple_test() {
        assert_eq!(
            do_codegen().to_string(),
            r#"fn print_hi () { println ! ("I guess this is working!") ; }"#
        );
    }

    // The expectorate crate makes for more comprehensible tests that are also
    // easier to review. When code generation changes, expectorate makes it
    // easy to update fixtures based on the new code.
    #[test]
    fn robust_test() {
        let code = do_codegen();
        let file = syn::parse2(code).expect("invalid codegen");
        let actual = prettyplease::unparse(&file);
        expectorate::assert_contents("tests/data/robust_test.rs", &actual);
    }
}
