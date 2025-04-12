extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr};

#[proc_macro]
pub fn rsx(input: TokenStream) -> TokenStream {
    let _input = parse_macro_input!(input as Expr);

    let output = quote! {
        FeNode::Element {
            tag: "div".into(),
            props: {
                let mut map = HashMap::new();
                map.insert("class".to_string(), "box".to_string());
                map
            },
            children: vec![
                FeNode::Element {
                    tag: "h1".into(),
                    props: HashMap::new(),
                    children: vec![
                        FeNode::Text("Hello World".into())
                    ]
                }
            ]
        }
    };

    output.into()
}


