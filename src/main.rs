use yew::prelude::*;

use {
    web_sys::{
        wasm_bindgen::{closure::Closure, JsCast},
        ErrorEvent, ErrorEventInit, WebSocket
    },
};


#[function_component(App)]
fn app() -> Html {
    let message = match ErrorEvent::new("error") {
        Err(e) => format!("Failed to create an ErrorEvent: {e:?}"),
        Ok(ee) => format!("message: {}", ee.message()),
    };
    html! {
        <>
            <h1>{ "Whee" }</h1>
            <p>{ message }</p>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
