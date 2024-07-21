use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
struct Pos {
    x: f64,
    y: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
struct Size {
    width: f64,
    height: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
struct Rect {
    pos: Pos,
    size: Size,
}

/// Operations that can be performed by the JS side.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
enum OutgoingMessage {
    /// Will mask/clip the given HTML element area.
    TransformElement {
        id: String,
        rect: Rect,
        holes: Vec<Rect>,
    },
    /// Request the current mouse position.
    MousePositionRequest,
    /// Logs to console.
    Log { message: String },
}

impl OutgoingMessage {
    /// Shorthand for sending the message to the JS side.
    fn send(&self) {
        send_message(serde_wasm_bindgen::to_value(self).unwrap());
    }

    /// Shorthand for calling `self.send()` and then `tick()`.
    fn send_and_tick(&self) {
        self.send();
        tick();
    }

    /// Shorthand for calling `self.send()`, then `tick()` and then `IncomingMessage::receive()`.
    fn send_and_tick_receiving(&self) -> IncomingMessage {
        self.send();
        tick();
        IncomingMessage::receive()
    }
}

/// Information received from the JS side.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
enum IncomingMessage {
    /// The mouse position received.
    MousePositionResponse { x: f64, y: f64 },
}

impl IncomingMessage {
    /// Deserializes the messages from a JS value.
    fn from_js_value(value: JsValue) -> Self {
        serde_wasm_bindgen::from_value(value).unwrap()
    }

    /// Receive a single message from the JS side.
    /// Panics if there are no messages or if there are more than one messages.
    fn receive() -> Self {
        let messages = receive_messages();
        let messages: Vec<IncomingMessage> = serde_wasm_bindgen::from_value(messages).unwrap();
        assert_eq!(messages.len(), 1);
        messages.into_iter().next().unwrap()
    }
}

#[wasm_bindgen(module = "/lib.js")]
extern "C" {
    /// Let the JS side do some work, consuming messages.
    #[wasm_bindgen]
    fn tick();

    /// Send a message to the JS side.
    #[wasm_bindgen(js_name = sendMessage)]
    fn send_message(msg: JsValue);

    /// Receives all messages from the JS side.
    #[wasm_bindgen(js_name = receiveMessages)]
    fn receive_messages() -> JsValue;
}

fn main() {
    console_error_panic_hook::set_once();

    let IncomingMessage::MousePositionResponse { x, y } =
        OutgoingMessage::MousePositionRequest.send_and_tick_receiving();

    OutgoingMessage::Log {
        message: format!("Mouse position: ({}, {})", x, y),
    }
    .send_and_tick();
}
