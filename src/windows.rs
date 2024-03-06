use std::thread;
use hookmap_core::event::Event;
use hookmap_core::mouse;

pub(crate) fn block_input() {
    thread::spawn(|| {
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
    });
}
