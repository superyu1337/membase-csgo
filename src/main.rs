use crate::core::structs::PlayerData;
use crate::core::structs::ThreadMsg;

use simplelog::SimpleLogger;

#[macro_use] extern crate log;
extern crate simplelog;

mod sdk;
mod core;
mod features;
mod menu;

fn main() {
    let _ = SimpleLogger::init(log::LevelFilter::Info, simplelog::Config::default());
    let (cheat_tx, menu_rx) = std::sync::mpsc::channel();
    let (menu_tx, cheat_rx) = std::sync::mpsc::channel();

    let mut cfg = core::structs::Config::default();

    // Initialize the cheat context. Gathers csgo process, modules and runs offset and netvar scans.
    let mut ctx = unsafe { core::setup(cfg)}
        .unwrap();

    let cheat_thread = std::thread::spawn(move || {
        let mut last_tick = 0;
        let mut average_execution_time: u128 = 0;
    
        loop {
            let start_instant = std::time::Instant::now();
            let global_vars = sdk::engine::get_globalvars(&mut ctx);
            let playerdata_array: [Option<PlayerData>; 64] = [None; 64];
    
            if global_vars.tickcount > last_tick {
                // Run your features here
    
                if cfg.glow {
                    features::glow::run(&mut ctx);
                }
            } else {
                let response = cheat_rx.try_recv();
                if response.is_ok() {
                    let msg: ThreadMsg = response.unwrap();

                    if msg.exited {
                        break;
                    }

                    if msg.new_config.is_some() {
                        cfg = msg.new_config.unwrap();
                    }
                }
            }

            cheat_tx.send(ThreadMsg {
                exited: false,
                new_config: None,
                playerdata_array: playerdata_array,
                average_execution_time: Some(average_execution_time)
            }).unwrap();
    
            last_tick = global_vars.tickcount;
            let end_instant = std::time::Instant::now();
            let execution_time = end_instant.duration_since(start_instant);
            average_execution_time += execution_time.as_nanos();
            average_execution_time /= 2;
            std::thread::sleep(std::time::Duration::from_micros(100));
        }
        info!("Average execution time: {} nanoseconds", average_execution_time);
    });

    menu::run_menu(menu_rx, menu_tx).unwrap();
    cheat_thread.join().unwrap();
}
