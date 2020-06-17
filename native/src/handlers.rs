mod handler {
}

use neon::prelude::*;

// A stub handler that contains the setup necessary to run a handler
// fn PLACEHOLDER_handler(o: Handle<JsObject>, mut cx: FunctionContext) {
//     let error_string = cx.string("none");
//     o.set(&mut cx, "error_string", error_string).unwrap();

//     // Do processing steps here
// }

pub fn unknown_handler(event_type: String, o: Handle<JsObject>, mut cx: FunctionContext) {
    let error_string = cx.string(format!("unknown event type: {}", event_type).as_str());
    o.set(&mut cx, "error_string", error_string).unwrap();
}