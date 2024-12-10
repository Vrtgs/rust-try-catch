use syn::{ExprClosure, ItemFn};
use syn::parse::{Parse, ParseStream};
use syn::parse::discouraged::Speculative;

pub(super) enum FnOrClosure {
    Function(ItemFn),
    Closure(ExprClosure),
}

impl Parse for FnOrClosure {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fork = input.fork();
        if let Ok(function) = fork.parse::<ItemFn>() {
            input.advance_to(&fork);
            return Ok(FnOrClosure::Function(function));
        }

        if let Ok(closure) = input.parse::<ExprClosure>() {
            return Ok(FnOrClosure::Closure(closure))
        }

        Err(input.error("Expected either a function or closure"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parsing() -> syn::Result<()> {
        let function_code = r#"
            fn example(a: i32) -> i32 {
                a + 1
            }
        "#;

        let closure_code = r#"
            |a: i32| a + 1
        "#;

        assert!(matches!(syn::parse_str(function_code)?, FnOrClosure::Function(_)));
        assert!(matches!(syn::parse_str(closure_code)?, FnOrClosure::Closure(_)));
        
        Ok(())
    }
}