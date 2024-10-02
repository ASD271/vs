use device_query::{DeviceEvents, DeviceState,MouseButton};
use neon::prelude::*;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn listen_mouse(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let callback = cx.argument::<JsFunction>(0)?.root(&mut cx);

    let (tx, rx) = mpsc::channel();

    let channel = cx.channel();

    let _producer = thread::spawn({
        let tx_clone=tx.clone();
        move || {
            let device_state = DeviceState::new();
            let _guard = device_state.on_mouse_up(move |button| {
                let message = match button {
                    1 => "left",
                    2 => "right",
                    3 => "middle",
                    _ => return
                };

                tx_clone.send(message).unwrap();
            });
            loop {
                thread::sleep(Duration::from_secs(1000)); //prevents the program from ending prematurely
            }
        }
    });

    channel.send(move |mut cx| {
        let this = cx.undefined();
        let callback = callback.into_inner(&mut cx);

        for r in rx {
            let args = vec![
                cx.null().upcast::<JsValue>(),
                cx.string(r).upcast::<JsValue>(),
            ];
            callback.call(&mut cx, this, args)?;
        }
        Ok(())
    });

    Ok(cx.undefined())
}
