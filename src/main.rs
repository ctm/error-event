use {
    web_sys::{
        wasm_bindgen::{closure::Closure, JsCast},
        ErrorEvent, WebSocket,
    },
    yew::prelude::*,
};

#[function_component(App)]
fn app() -> Html {
    // WebSocket::new does not attempt to make the connection before
    // returning, so the unwrap() here is not related to whether or
    // not the connection succeeds.
    let ws = WebSocket::new("wss://example.com/not-available").unwrap();

    // This is largely the same code from wasm-sockets, because it's
    // that code that I'm having trouble with.  The problem isn't with
    // wasm-sockets.  The problem is that the ErrorEvent::message
    // can't be called on then ErrorEvent that is passed into the
    // onerror handler.  I've also tried some a similar methods,
    // ErrorEvent::filename, and it too had trouble.
    let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
        e.message(); // This causes the program to crash, which can be seen
                     // by looking in the JavaScript console.
        // e.filename(); // as does this
    }) as Box<dyn Fn(ErrorEvent)>);
    ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
    onerror_callback.forget();
    html! {
        <p>
            { "Open your JavaScript console and look for errors, please." }
        </p>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
