use std::collections::HashMap;

use dataview::Pod;
use memflow::{VirtualDMA, ConnectorInstance, DirectTranslate};
use memflow_win32::{Win32Process, Win32VirtualTranslate, Win32ModuleInfo};

use crate::sdk::structs::vec3::Vec3;

pub struct CheatCtx {
    pub process: Win32Process<VirtualDMA<ConnectorInstance, DirectTranslate, Win32VirtualTranslate>>,
    pub engine_module: Win32ModuleInfo,
    pub client_module: Win32ModuleInfo,
    pub offsets: HashMap<String, usize>,
    pub config: Config,
}

#[derive(Pod, Clone, Copy)]
#[repr(C)]
pub struct Config {
    pub glow: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct PlayerData {
    pub name: [char; 128],
    pub rank: i32,
    pub rank_name: [char; 32],
    pub pos: Vec3,
    pub team: i32
}

impl PlayerData {
    pub fn new_invalid() -> PlayerData {
        PlayerData {
            name: ['0'; 128],
            rank: -1,
            rank_name: ['0'; 32],
            pos: Vec3::new(0.0, 0.0, 0.0),
            team: -1,
        }
    }
}

#[derive(Clone, Copy)]
pub struct ThreadMsg {
    pub new_config: Config,
    pub playerdata_array: [PlayerData; 64]
}