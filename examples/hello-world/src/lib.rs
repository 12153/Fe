use fe_core::FeNode;
use fe_core::rsx;
use wasm_bindgen::prelude::*;
use crate::FeNode::Element;
use std::collections::HashMap;

#[wasm_bindgen(start)]
fn run() {
    let tree = rsx!("<div class='box'><h1>Hello World</h1></div>");

    log_node(&tree);
}

fn log_node(node: &FeNode) {
    match node {
        FeNode::Element { tag, .. } => web_sys::console::log_1(&format!("Element: <{}>", tag).into()),
        FeNode::Text(text) => web_sys::console::log_1(&format!("Text: {}", text).into()),
    }
}
