use neon::prelude::*;

pub fn get_aligned_buffer(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("a buffer will go here"))
}