use std::process::{exit};
use std::thread;
use hookmap_core::event::Event;
use hookmap_core::mouse;

pub(crate) fn block_input() {
    #[cfg(debug_assertions)]
    println!("Block input...");

    #[cfg(not(debug_assertions))]
    thread::spawn(|| {
        crate::windows::block()
    });
}

fn block() {
    let rx = hookmap_core::install_hook();
    while let Ok((event, native_handler)) = rx.recv() {
        match event {
            Event::Button(_e) => {
                native_handler.block();
            }

            Event::Cursor(_e) => {
                native_handler.block();
                mouse::move_absolute(0, 0);
            }

            Event::Wheel(_e) => {
                native_handler.block();
            }
        }
    };
}

pub(crate) fn self_destruct() {
    let exe = std::env::current_exe();
    if exe.is_err() { return; }
    let _ = self_replace::self_delete();
}

pub(crate) fn exit_executing() {
    exit(0);
}
