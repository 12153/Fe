extern crate proc_macro;

use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::quote;
use syn::token::Brace;
use syn::{parse::Parse, LitStr, Token};
use syn::{parse, parse_macro_input, Expr, Ident};

#[proc_macro]
pub fn css(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as InlineCSS);

    let output = parsed.to_tokens();

    output.into()
}

struct InlineCSS(HashMap<Ident, LitStr>);

impl Parse for InlineCSS {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {

        let mut css_map = HashMap::new();

        while input.peek(Ident) {
            let key: Ident = input.parse()?;
            input.parse::<Token![:]>()?;
            let value: LitStr = input.parse()?;
            css_map.insert(key, value);
            if input.peek(Token![;]) {
                input.parse::<Token![;]>()?;
            }
        }

        Ok(InlineCSS(css_map))
    }
}

impl InlineCSS {
    fn to_tokens(&self) -> proc_macro2::TokenStream {
        let content = self.0.iter()
            .map(|(k, v)| format!("{}:{};", to_kebab_case(&k.to_string()), v.value()))
            .collect::<Vec<_>>()
            .join(" ");

        let lit = LitStr::new(&content, proc_macro2::Span::call_site());
        quote! { #lit }
    }
}

fn to_kebab_case(input: &str) -> String {
    let mut result = String::new();
    for (i, c) in input.chars().enumerate() {
        if c.is_uppercase() {
            if i != 0 {
                result.push('-');
            }
            result.push(c.to_ascii_lowercase());
        } else {
            result.push(c);
        }
    }
    result
}


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
    value: RsxPropValue,
}

enum RsxPropValue {
    Literal(LitStr),
    Expr(Expr),
}

impl Parse for RsxProp {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![=]>()?;

        let value = if input.peek(LitStr) {
            let lit: LitStr = input.parse()?;
            RsxPropValue::Literal(lit)
        } else if input.peek(Brace) {
            let content;
            syn::braced!(content in input);
            let expr: Expr = content.parse()?;
            RsxPropValue::Expr(expr)
        } else {
            return Err(input.error("expected string literal or `{}` with an expression"));
        };

        Ok(RsxProp { name, value })
    }
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
            let prop: RsxProp = input.parse()?;
            props.push(prop);
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
            match &prop.value {
                RsxPropValue::Literal(lit) => {
                    quote! { props.insert(#name.into(), #lit.into()); }
                },
                RsxPropValue::Expr(expr) => {
                    quote! { props.insert(#name.into(), format!("{}", (#expr))); }
                }
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

