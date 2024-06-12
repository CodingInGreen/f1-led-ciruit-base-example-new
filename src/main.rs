#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use esp_backtrace as _;
use esp_hal::prelude::*;

#[embassy_executor::task]
async fn run() {
    
}

#[main]
async fn main(spawner: Spawner) {
    esp_println::println!("Hello World!");

    spawner.spawn(run()).ok();

    loop {

    }
}