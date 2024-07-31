# ErrorEvent Trouble

This is a "minimal" test case that shows that in certain
circumstances, an `ErrorEvent::message` invocation will cause WASM
running in a web browser to fail in an unrecoverable
way. Specifically, the `ErrorEvent` passed to the `onerror` handler of
a `WebSocket` will not be able to call `message()`.

## To reproduce

Install trunk and then have trunk run the program:

```
$ cargo install trunk
$ trunk serve --open
```

Now open up your JavaScript console and you should see something like:

```
09:38:42.243 WebSocket connection to 'wss://example.com/not-available' failed: 
09:38:42.243 Uncaught
TypeError: Cannot read properties of undefined (reading 'length')
    at passStringToWasm0 (error-event-8348614ec5312283.js:142:19)
    at imports.wbg.__wbg_message_730094b985f3ba2e (error-event-8348614ec5312283.js:485:22)
    at error_event-77a5375a3ee8a4dc.wasm.web_sys::features::gen_ErrorEvent::ErrorEvent::message::h53ae803a5f72d5d2 (error-event-8348614ec5312283_bg.wasm:0xb3c8c)
    at error_event-77a5375a3ee8a4dc.wasm.<error_event::App as yew::functional::FunctionProvider>::run::app::{{closure}}::hea26a0e2e6a812a6 (error-event-8348614ec5312283_bg.wasm:0xbb828)
    at error_event-77a5375a3ee8a4dc.wasm.<dyn core::ops::function::Fn<(:8080/A,)>+Output = R as wasm_bindgen::closure::WasmClosure>::describe::invoke::h826fadf1d4a4f6fd (http://127.0.0.1:8080/error-event-8348614ec5312283_bg.wasm)
    at __wbg_adapter_20 (error-event-8348614ec5312283.js:208:10)
    at WebSocket.real (error-event-8348614ec5312283.js:194:20)

```

The first complaint "WebSocket connection ... failed" is just a warning and,
although annoying to some, doesn't prevent the WASM from continuing.  The
second error is the problem.

## Trunk and Yew

I realize that by requiring trunk and yew as dependencies that this
isn't really minimal. However, I am not particularly familiar with
JavaScript ecosystem particulars.  My company's (closed-source)
software uses yew with webpack on the front-end and actix-web on the
back-end.  So, this same behavior happens with webpack and trunk,
which makes me guess that this is problem is at the web-sys level.

[Cliff Matthews](mailto:clifford.t.matthews@gmail.com)
