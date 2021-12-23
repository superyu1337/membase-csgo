use crate::core::structs::PlayerData;
use std::{net::TcpListener};

use crate::core::structs::ThreadMsg;

use simplelog::SimpleLogger;

#[macro_use] extern crate log;
extern crate simplelog;

mod sdk;
mod core;
mod features;

fn main() {
    let _ = SimpleLogger::init(log::LevelFilter::Debug, simplelog::Config::default());

    let mut cfg = core::structs::Config {
        glow: 0,
    };

    let mut ctx = unsafe { core::setup(cfg)}
        .unwrap();

    let (socket_tx, cheat_rx) = std::sync::mpsc::channel();

    // Websocket thread
    let _websocket_thread = std::thread::spawn(move || {
        // shitty loop here

        let server = TcpListener::bind("0.0.0.0:42069").expect("Bingind WebSocket");
        for stream in server.incoming() {

            let tx = socket_tx.clone();

            std::thread::spawn(move || {
                let mut websocket = tungstenite::accept(stream.unwrap()).unwrap();
                
                loop {
                    let msg = websocket.read_message().unwrap();

                    if !msg.is_binary() {
                        continue;
                    }

                    let mut msg_content = msg.into_data();

                    match msg_content[0] {
                        // Sending a new configuration
                        0x01 => {
                            msg_content.remove(0);
                            let new_config = core::structs::Config {
                                    glow: msg_content[0]
                            };

                            tx.send(ThreadMsg {
                                new_config,
                                playerdata_array: [PlayerData::new_invalid(); 64]
                            }).expect("Sending message to cheat thread");
                        },
                        _ => {
                            debug!("Unrecognized message received: {:#?}", msg_content);
                        }
                    }
                }
            });
        }

    });

    let mut last_tick = 0;

    loop {
        let global_vars = sdk::engine::get_globalvars(&mut ctx);
        let _playerdata_array = [PlayerData::new_invalid(); 64];

        if global_vars.tickcount > last_tick {
            // run your cheat stuff
            if cfg.glow == 1 {
                features::glow::run(&mut ctx);
            }
        } else {
            let response = cheat_rx.recv_timeout(std::time::Duration::from_millis(1));

            if response.is_ok() {
                let thread_msg: ThreadMsg = response.unwrap();
                cfg = thread_msg.new_config;
            }
        }

        last_tick = global_vars.tickcount;
    }
}
