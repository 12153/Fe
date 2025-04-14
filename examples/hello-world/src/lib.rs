use fe_core::{FeNode, render};
use fe_core::{rsx, css};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn start() {
    let document = web_sys::window().unwrap().document().unwrap();
    let app = document.get_element_by_id("app").unwrap();

    let tree = rsx! { 
        <div class="box" style={css! {
            backgroundColor: "red";
            width: "100%";
            height: "200px";
            padding: "8px";
        }}>
            <h1>"Hello World"</h1>
            <p>"this is really cool"</p>
        </div>
    };

    let node = render(&document, &tree);
    app.append_child(&node).expect("could not append child");
}

