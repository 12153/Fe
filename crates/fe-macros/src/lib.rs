extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, LitStr, Token};
use syn::{parse, parse_macro_input, Ident};

#[proc_macro]
pub fn rsx(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as RsxElement);

    let output = parsed.to_tokens();
    
    output.into()
}

enum RsxNode {
    Element(RsxElement),
    Text(LitStr),
}

struct RsxElement {
    tag: Ident,
    props: Vec<RsxProp>,
    children: Vec<RsxNode>,
}

struct RsxProp {
    name: Ident,
    value: LitStr,
}

impl Parse for RsxElement {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        // Parse opening '<'
        input.parse::<Token![<]>()?;

        // Parse tag name
        let tag: Ident = input.parse()?;

        // Parse props
        let mut props = Vec::new();
        while input.peek(Ident) && input.peek2(Token![=]) {
            let name: Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            let value: LitStr = input.parse()?;
            props.push(RsxProp { name, value });
        }

        // Parse '>'
        input.parse::<Token![>]>()?;

        // Parse children
        let mut children = Vec::new();
        while !(input.peek(Token![<]) && input.peek2(Token![/])) {
            if input.peek(LitStr) {
                let text: LitStr = input.parse()?;
                children.push(RsxNode::Text(text));
            } else if input.peek(Token![<]) {
                children.push(RsxNode::Element(input.parse()?));
            } else {
                return Err(input.error("unexpected token in children"));
            }
        }

        // Parse closing `</tag>`
        input.parse::<Token![<]>()?;
        input.parse::<Token![/]>().expect("error parsing / ");
        let end_tag: Ident = input.parse()?;
        if end_tag != tag {
            return Err(input.error("mismatched closing tag"));
        }
        input.parse::<Token![>]>()?;

        Ok(RsxElement { tag, props, children })
    }
}


impl RsxElement {
    pub fn to_tokens(&self) -> proc_macro2::TokenStream {
        let tag = self.tag.to_string();

        // Props: turn Vec<RsxProp> into key-value inserts
        let props_inserts = self.props.iter().map(|prop| {
            let name = prop.name.to_string();
            let value = prop.value.value();
            quote! {
                props.insert(#name.into(), #value.into());
            }
        });

        // Children: turn Vec<RsxNode> into FeNode::Text / FeNode::Element
        let children_tokens = self.children.iter().map(|child| {
            match child {
                RsxNode::Text(text) => {
                    let text_val = text.value();
                    quote! {
                        FeNode::Text(#text_val.into())
                    }
                }
                RsxNode::Element(el) => {
                    let el_tokens = el.to_tokens();
                    quote! {
                        #el_tokens
                    }
                }
            }
        });

        quote! {
            {
                let mut props = std::collections::HashMap::new();
                #(#props_inserts)*

                FeNode::Element {
                    tag: #tag.into(),
                    props,
                    children: vec![
                        #(#children_tokens),*
                    ],
                }
            }
        }
    }
}

