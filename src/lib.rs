extern crate js_sys;
extern crate wasm_bindgen;

#[macro_use]
extern crate serde_json;

use aws_lambda_events::event::apigw::*;
use js_sys::Promise;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[allow(unused_variables)]
pub fn hello(event: JsValue, context: JsValue) -> Promise {
    let http_event: ApiGatewayProxyRequest = event.into_serde().unwrap();
    let data = json!({
        "message": "Hello, WebAssembly!",
        "input": http_event
    });

    let response = Response {
        statusCode: 200,
        body: serde_json::to_string_pretty(&data).unwrap(),
    };
    let js_result = serde_wasm_bindgen::to_value(&response).unwrap();
    return Promise::resolve(&js_result);
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Response {
    pub statusCode: u16,
    pub body: String,
}
