use std::thread;
use std::time::Duration;
use neon::prelude::*;

pub fn parse_async(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let input = cx.argument::<JsString>(0)?.value(&mut cx);
    let callback = cx.argument::<JsFunction>(1)?.root(&mut cx);
    let channel = cx.channel();

    std::thread::spawn(move || {
        parse(input, callback, channel);
    });

    Ok(cx.undefined())
}

fn parse(input: String, callback: Root<JsFunction>, channel: Channel) {
    let result = process_input(input);

    channel.send(move |mut cx| {
        let callback = callback.into_inner(&mut cx);
        let this = cx.undefined();
        // let args = vec![cx.undefined().upcast::<JsValue>()];
        let args = match result {
            Ok(success) => vec![cx.null().upcast::<JsValue>(), cx.string(success).upcast()],
            Err(error) => vec![cx.string(error).upcast(), cx.undefined().upcast()],
        };
        callback.call(&mut cx, this, args)?;
        let args=vec![cx.null().upcast::<JsValue>(),cx.string("final").upcast()];
        callback.call(&mut cx,this,args)?;
        Ok(())
    });
}

fn process_input(input: String) -> Result<String, String> {
    // 模拟处理过程
    thread::sleep(Duration::from_secs(2));
    Ok(format!("处理结果: {}", input.to_uppercase()))
}
