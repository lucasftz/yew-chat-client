use gloo::console::log;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::WebSocket;

#[derive(Clone, PartialEq)]
pub struct Socket {
    ws: Option<WebSocket>,
}

impl Socket {
    pub fn new() -> Self {
        Socket { ws: None }
    }

    pub fn connect(&mut self, url: &str) {
        let ws = match WebSocket::new(url) {
            Ok(ws) => ws,
            Err(_) => panic!("Could not connect to websocket"),
        };

        let onopen = {
            let ws = ws.clone();
            Closure::<dyn FnMut()>::new(move || {
                log!("socket opened");
                match ws.send_with_u8_array(b"Hello world") {
                    Ok(_) => log!("ut8 message successfully sent"),
                    Err(err) => log!("error sending message: {:?}", err),
                }
            })
        };

        ws.set_onopen(Some(onopen.as_ref().unchecked_ref()));
        onopen.forget();

        self.ws = Some(ws);
    }
}
