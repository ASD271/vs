extern crate device_query;

use device_query::{DeviceEvents, DeviceState};
use std::thread;
use std::time::Duration;
use tokio::runtime::Runtime;

fn main() {
    tokio_listen();
}
fn thread_listen(){
    let device_state = DeviceState::new();
    let _guard = device_state.on_mouse_move(|position| {
        println!("Position: {:#?}", position);
    });
    let _guard = device_state.on_mouse_down(|button| {
        println!("Down: {:#?}", button);
    });
    let _guard = device_state.on_mouse_up(|button| {
        println!("Up: {:#?}", button);
    });

    loop {
        thread::sleep(Duration::from_secs(1000));
    }
}
fn tokio_listen(){
    let rt = Runtime::new().unwrap();
    rt.spawn(async move {
        let device_state = DeviceState::new();
        let _guard = device_state.on_mouse_up(move |button| {
            let message = match button {
                1 => "left",
                2 => "right",
                3 => "middle",
                _ => return,
            };

            print!("{}",message);
        });
        // let mut i = 1;
        // loop {
        //     // tx.send(format!("hello{i}")).unwrap();
        //     tokio::time::sleep(Duration::from_secs(3)).await;
        //     i += 1;
        //     if i > 5 {
        //         break;
        //     }
        // }
        // tx.send(format!("end")).unwrap();
    });
}