use fe_core::{FeNode, render};
use fe_core::rsx;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen(start)]
fn start() {
    let document = web_sys::window().unwrap().document().unwrap();
    let app = document.get_element_by_id("app").unwrap();

    console::log_1(&"running...".into());

    let tree = rsx! { 
        <div class="box">
            <h1>"Hello World"</h1>
            <p>"this is really cool"</p>
        </div>
    };

    let node = render(&document, &tree);
    app.append_child(&node).expect("could not append child");
}

