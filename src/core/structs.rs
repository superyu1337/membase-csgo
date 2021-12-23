use std::collections::HashMap;

use memflow::{VirtualDMA, ConnectorInstance, DirectTranslate, CachedMemoryAccess, TimedCacheValidator, CachedVirtualTranslate};
use memflow_win32::{Win32Process, Win32ModuleInfo, Win32VirtualTranslate};

use crate::sdk::structs::vec3::Vec3;

pub struct CheatCtx<'a> {
    pub process: Win32Process<VirtualDMA<CachedMemoryAccess<'a, ConnectorInstance, TimedCacheValidator>, CachedVirtualTranslate<DirectTranslate, TimedCacheValidator>, Win32VirtualTranslate>>,
    pub engine_module: Win32ModuleInfo,
    pub client_module: Win32ModuleInfo,
    pub offsets: HashMap<String, usize>,
    pub config: Config,
}

#[derive(Clone, Copy)]
pub struct Config {
    pub glow: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct PlayerData {
    pub name: [char; 128],
    pub rank: i32,
    pub rank_name: [char; 32],
    pub pos: Vec3,
    pub team: i32
}

#[derive(Clone, Copy)]
pub struct ThreadMsg {
    pub exited: bool,
    pub new_config: Option<Config>,
    pub playerdata_array: Option<[PlayerData; 64]>
}