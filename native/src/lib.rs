pub mod handlers;

use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

fn process_request(mut cx: FunctionContext) -> JsResult<JsObject> {
    let event_type: String = cx.argument::<JsString>(0)?.value();

    let o: Handle<JsObject> = JsObject::new(&mut cx);

    match event_type.as_str() {
        // This needs to always be the last line of the match
        _ => handlers::unknown_handler(event_type, o, cx)
    };

    Ok(o)
}

register_module!(mut cx, {
    cx.export_function("hello", hello)?;
    cx.export_function("process_request", process_request)?;
    Ok(())
});
