use std::collections::HashMap;

pub enum FeNode {
   Element {
    tag: String,
    props: HashMap<String, String>,
    children: Vec<FeNode>,
   },
   Text(String),
}

pub use fe_macros::rsx;
