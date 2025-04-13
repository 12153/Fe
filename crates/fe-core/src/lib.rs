use std::collections::HashMap;
use web_sys::{Document, Node, console};

pub enum FeNode {
   Element {
    tag: String,
    props: HashMap<String, String>,
    children: Vec<FeNode>,
   },
   Text(String),
}

pub fn render(document: &Document, node: &FeNode) -> Node {
    console::log_1(&"Rendering...".into());
    match node {
        FeNode::Element { tag, props, children } => {
            let el = document.create_element(tag).unwrap();

            for (key, value) in props {
                el.set_attribute(key, value).unwrap();
            }

            for child in children {
                let child_node = render(document, child);
                el.append_child(&child_node).unwrap();
            }

            el.into()
        }
        FeNode::Text(text) => document.create_text_node(text).into()
    }
}

pub use fe_macros::rsx;
