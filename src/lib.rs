#[macro_use]
extern crate serde_json;

use aws_lambda_events::event::apigw::*;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
#[allow(unused_variables)]
pub fn hello(event: JsValue, context: JsValue) -> Promise {
    console::log_1(&event);
    let http_event: ApiGatewayProxyRequest = event.into_serde().unwrap(); // convert JsValue to Rust type
    let data = json!({
        "message": "Hello, WebAssembly!",
        "input": http_event
    });

    let response = ApiGatewayProxyResponse {
        status_code: 200,
        body: serde_json::to_string_pretty(&data).ok(),
        headers: Default::default(),
        multi_value_headers: Default::default(),
        is_base64_encoded: None,
    };
    let js_result = serde_wasm_bindgen::to_value(&response).unwrap(); // convert Rust type to JsValue
    return Promise::resolve(&js_result);
}
