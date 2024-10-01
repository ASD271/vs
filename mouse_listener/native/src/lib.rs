use std::fmt::Debug;
use neon::prelude::*;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;
mod hello;
mod parse_async;

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    use hello::hello;
    use parse_async::parse_async;
    cx.export_function("hello",hello)?;
    cx.export_function("parseAsync", parse_async)?;
    Ok(())
}