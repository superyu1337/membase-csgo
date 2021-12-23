use std::net::TcpListener;

//use crate::core::structs::PlayerData;

use crate::core::structs::ThreadMsg;

use simplelog::SimpleLogger;

#[macro_use] extern crate log;
extern crate simplelog;

mod sdk;
mod core;
mod features;

fn main() {
    let _ = SimpleLogger::init(log::LevelFilter::Info, simplelog::Config::default());

    let mut cfg = core::structs::Config {
        glow: true,
    };

    let mut ctx = unsafe { core::setup(cfg)}
        .unwrap();

    let (socket_tx, cheat_rx) = std::sync::mpsc::channel();


    let cheat_thread = std::thread::spawn(move || {
        let mut last_tick = 0;
        let mut average_execution_time: u128 = 0;

        loop {
            let start_instant = std::time::Instant::now();
            let global_vars = sdk::engine::get_globalvars(&mut ctx);
            //let playerdata_array: Option<[PlayerData; 64]> = None;

            if global_vars.tickcount > last_tick {
                // Run your features here

                if cfg.glow {
                    features::glow::run(&mut ctx);
                }
            } else {
                let response = cheat_rx.try_recv();

                if response.is_ok() {
                    let thread_msg: ThreadMsg = response.unwrap();

                    if thread_msg.exited {
                        break;
                    }

                    cfg = thread_msg.new_config.unwrap();
                }
            }

            last_tick = global_vars.tickcount;

            let end_instant = std::time::Instant::now();
            let execution_time = end_instant.duration_since(start_instant);
            average_execution_time += execution_time.as_nanos();
            average_execution_time /= 2;
        }
        info!("Average execution time: {} nanoseconds", average_execution_time);
    });

    // Websocket thread
    let websocket_thread = std::thread::spawn(move || {
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
                                    glow: msg_content[0] != 0
                            };

                            tx.send(ThreadMsg {
                                exited: false,
                                new_config: Some(new_config),
                                playerdata_array: None
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

    cheat_thread.join().unwrap();
    websocket_thread.join().unwrap();
}
